use std::collections::BTreeMap;
use std::sync::{Arc, mpsc};

use anyhow::Result;
use depot::{
	conveyer::{Db, db::CompactionSignaler},
	error::SqliteStorageError,
	workflows::compaction::DeltasAvailable,
};
use parking_lot::Mutex;
use rivet_envoy_protocol as protocol;
use rivet_pools::{__rivet_util::Id, NodeId};

pub(crate) struct DirectStorage {
	db: Arc<universaldb::Database>,
	node_id: NodeId,
	actor_dbs: scc::HashMap<String, Arc<Db>>,
	page_mirrors: scc::HashMap<String, Arc<Mutex<DirectActorPages>>>,
	compaction_signals: Arc<Mutex<Vec<DeltasAvailable>>>,
	pub(crate) hooks: Arc<DirectTransportHooks>,
}

#[derive(Clone, Default)]
pub(crate) struct DirectActorPages {
	pub(crate) db_size_pages: u32,
	pub(crate) pages: BTreeMap<u32, Vec<u8>>,
}

impl DirectStorage {
	pub(crate) fn new(db: universaldb::Database) -> Self {
		Self {
			db: Arc::new(db),
			node_id: NodeId::new(),
			actor_dbs: scc::HashMap::new(),
			page_mirrors: scc::HashMap::new(),
			compaction_signals: Arc::new(Mutex::new(Vec::new())),
			hooks: Arc::new(DirectTransportHooks::default()),
		}
	}

	pub(crate) async fn actor_db(&self, actor_id: String) -> Arc<Db> {
		let signals = Arc::clone(&self.compaction_signals);
		self.actor_dbs
			.entry_async(actor_id.clone())
			.await
			.or_insert_with(|| {
				let compaction_signaler: CompactionSignaler = Arc::new(move |signal| {
					let signals = Arc::clone(&signals);
					Box::pin(async move {
						signals.lock().push(signal);
						Ok(())
					})
				});
				Arc::new(Db::new_with_compaction_signaler(
					Arc::clone(&self.db),
					Id::nil(),
					actor_id,
					self.node_id,
					None,
					compaction_signaler,
				))
			})
			.get()
			.clone()
	}

	async fn page_mirror(&self, actor_id: String) -> Arc<Mutex<DirectActorPages>> {
		self.page_mirrors
			.entry_async(actor_id)
			.await
			.or_insert_with(|| Arc::new(Mutex::new(DirectActorPages::default())))
			.get()
			.clone()
	}

	pub(crate) async fn get_pages(
		&self,
		actor_id: &str,
		pgnos: &[u32],
	) -> anyhow::Result<Vec<depot::types::FetchedPage>> {
		let actor_db = self.actor_db(actor_id.to_string()).await;
		match actor_db.get_pages(pgnos.to_vec()).await {
			Ok(pages) => Ok(self.fill_from_mirror(actor_id, pgnos, pages).await),
			Err(err) => {
				if matches!(
					depot_error(&err),
					Some(SqliteStorageError::MetaMissing { operation })
						if *operation == "get_pages"
				) {
					Ok(self.read_mirror(actor_id, pgnos).await)
				} else {
					Err(err)
				}
			}
		}
	}

	async fn fill_from_mirror(
		&self,
		actor_id: &str,
		pgnos: &[u32],
		pages: Vec<depot::types::FetchedPage>,
	) -> Vec<depot::types::FetchedPage> {
		let mut by_pgno = pages
			.into_iter()
			.map(|page| (page.pgno, page))
			.collect::<BTreeMap<_, _>>();
		let mirror_pages = self.read_mirror(actor_id, pgnos).await;
		for page in mirror_pages {
			if page.bytes.is_some()
				|| by_pgno.get(&page.pgno).is_none_or(|existing| existing.bytes.is_none())
			{
				by_pgno.insert(page.pgno, page);
			}
		}
		pgnos
			.iter()
			.map(|pgno| {
				by_pgno.remove(pgno).unwrap_or(depot::types::FetchedPage {
					pgno: *pgno,
					bytes: None,
				})
			})
			.collect()
	}

	async fn read_mirror(
		&self,
		actor_id: &str,
		pgnos: &[u32],
	) -> Vec<depot::types::FetchedPage> {
		let mirror = self.page_mirror(actor_id.to_string()).await;
		let mirror = mirror.lock();
		pgnos
			.iter()
			.map(|pgno| depot::types::FetchedPage {
				pgno: *pgno,
				bytes: if *pgno <= mirror.db_size_pages {
					mirror.pages.get(pgno).cloned()
				} else {
					None
				},
			})
			.collect()
	}

	pub(crate) async fn apply_commit(
		&self,
		actor_id: &str,
		dirty_pages: Vec<depot::types::DirtyPage>,
		db_size_pages: u32,
	) {
		let mirror = self.page_mirror(actor_id.to_string()).await;
		let mut mirror = mirror.lock();
		mirror.db_size_pages = db_size_pages;
		mirror.pages.retain(|pgno, _| *pgno <= db_size_pages);
		for page in dirty_pages {
			mirror.pages.insert(page.pgno, page.bytes);
		}
	}

	pub(crate) async fn snapshot_pages(&self, actor_id: &str) -> DirectActorPages {
		self.page_mirror(actor_id.to_string()).await.lock().clone()
	}

	pub(crate) fn compaction_signals(&self) -> Vec<DeltasAvailable> {
		self.compaction_signals.lock().clone()
	}
}

#[derive(Default)]
pub(crate) struct DirectTransportHooks {
	fail_next_commit: Mutex<Option<String>>,
	pause_next_commit: Mutex<Option<DirectCommitGate>>,
}

impl DirectTransportHooks {
	pub(crate) fn fail_next_commit(&self, message: impl Into<String>) {
		*self.fail_next_commit.lock() = Some(message.into());
	}

	pub(crate) fn pause_next_commit(&self) -> DirectCommitPause {
		let (reached_tx, reached_rx) = mpsc::channel();
		let (resume_tx, resume_rx) = mpsc::channel();
		*self.pause_next_commit.lock() = Some(DirectCommitGate {
			reached: reached_tx,
			resume: resume_rx,
		});
		DirectCommitPause {
			reached: reached_rx,
			resume: resume_tx,
		}
	}

	pub(crate) fn take_commit_error(&self) -> Option<String> {
		self.fail_next_commit.lock().take()
	}

	pub(crate) fn pause_commit_if_requested(&self) {
		let Some(gate) = self.pause_next_commit.lock().take() else {
			return;
		};
		let _ = gate.reached.send(());
		let _ = gate.resume.recv();
	}
}

pub(crate) struct DirectCommitPause {
	reached: mpsc::Receiver<()>,
	resume: mpsc::Sender<()>,
}

impl DirectCommitPause {
	pub(crate) fn wait_until_reached(&self) {
		self.reached
			.recv()
			.expect("commit pause should be reached");
	}

	pub(crate) fn resume(self) {
		self.resume.send(()).expect("commit pause should resume");
	}
}

struct DirectCommitGate {
	reached: mpsc::Sender<()>,
	resume: mpsc::Receiver<()>,
}

pub(crate) fn protocol_fetched_page(page: depot::types::FetchedPage) -> protocol::SqliteFetchedPage {
	protocol::SqliteFetchedPage {
		pgno: page.pgno,
		bytes: page.bytes,
	}
}

pub(crate) fn storage_dirty_page(page: protocol::SqliteDirtyPage) -> depot::types::DirtyPage {
	depot::types::DirtyPage {
		pgno: page.pgno,
		bytes: page.bytes,
	}
}

fn depot_error(err: &anyhow::Error) -> Option<&SqliteStorageError> {
	err.downcast_ref::<SqliteStorageError>()
}

fn sqlite_error_reason(err: &anyhow::Error) -> String {
	err.chain()
		.map(ToString::to_string)
		.collect::<Vec<_>>()
		.join(": ")
}

pub(crate) fn sqlite_error_response(err: &anyhow::Error) -> protocol::SqliteErrorResponse {
	protocol::SqliteErrorResponse {
		message: sqlite_error_reason(err),
	}
}

pub(crate) struct MockProtocol {
	commit_behavior: Mutex<MockCommitBehavior>,
	pub(crate) get_pages_response: protocol::SqliteGetPagesResponse,
	commit_requests: Mutex<Vec<protocol::SqliteCommitRequest>>,
	get_pages_requests: Mutex<Vec<protocol::SqliteGetPagesRequest>>,
}

#[derive(Clone)]
enum MockCommitBehavior {
	Respond(protocol::SqliteCommitResponse),
	Pending,
}

impl MockProtocol {
	pub(crate) fn new(commit_response: protocol::SqliteCommitResponse) -> Self {
		Self {
			commit_behavior: Mutex::new(MockCommitBehavior::Respond(commit_response)),
			get_pages_response: protocol::SqliteGetPagesResponse::SqliteGetPagesOk(
				protocol::SqliteGetPagesOk {
					pages: vec![],
				},
			),
			commit_requests: Mutex::new(Vec::new()),
			get_pages_requests: Mutex::new(Vec::new()),
		}
	}

	pub(crate) fn hang_commits(&self) {
		*self.commit_behavior.lock() = MockCommitBehavior::Pending;
	}

	pub(crate) fn commit_requests(
		&self,
	) -> parking_lot::MutexGuard<'_, Vec<protocol::SqliteCommitRequest>> {
		self.commit_requests.lock()
	}

	pub(crate) fn get_pages_requests(
		&self,
	) -> parking_lot::MutexGuard<'_, Vec<protocol::SqliteGetPagesRequest>> {
		self.get_pages_requests.lock()
	}

	pub(crate) async fn get_pages(
		&self,
		req: protocol::SqliteGetPagesRequest,
	) -> Result<protocol::SqliteGetPagesResponse> {
		self.get_pages_requests().push(req);
		Ok(self.get_pages_response.clone())
	}

	pub(crate) async fn commit(
		&self,
		req: protocol::SqliteCommitRequest,
	) -> Result<protocol::SqliteCommitResponse> {
		let req = req.clone();
		self.commit_requests().push(req.clone());
		match self.commit_behavior.lock().clone() {
			MockCommitBehavior::Respond(response) => Ok(response),
			MockCommitBehavior::Pending => std::future::pending().await,
		}
	}
}
