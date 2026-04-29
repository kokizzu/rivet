#[cfg(debug_assertions)]
use std::sync::Arc;

#[cfg(debug_assertions)]
use parking_lot::Mutex;
#[cfg(debug_assertions)]
use tokio::sync::Notify;

use super::*;

#[cfg(debug_assertions)]
static PAUSE_AFTER_HOT_STAGE: Mutex<Option<(DatabaseBranchId, Arc<Notify>, Arc<Notify>)>> =
	Mutex::new(None);

#[cfg(debug_assertions)]
pub struct PauseGuard {
	slot: &'static Mutex<Option<(DatabaseBranchId, Arc<Notify>, Arc<Notify>)>>,
}

#[cfg(debug_assertions)]
pub fn pause_after_hot_stage(
	database_branch_id: DatabaseBranchId,
) -> (PauseGuard, Arc<Notify>, Arc<Notify>) {
	let reached = Arc::new(Notify::new());
	let release = Arc::new(Notify::new());
	*PAUSE_AFTER_HOT_STAGE.lock() =
		Some((database_branch_id, Arc::clone(&reached), Arc::clone(&release)));

	(
		PauseGuard {
			slot: &PAUSE_AFTER_HOT_STAGE,
		},
		reached,
		release,
	)
}

#[cfg(debug_assertions)]
pub(super) async fn maybe_pause_after_hot_stage(database_branch_id: DatabaseBranchId) {
	let hook = PAUSE_AFTER_HOT_STAGE
		.lock()
		.as_ref()
		.filter(|(hook_branch_id, _, _)| *hook_branch_id == database_branch_id)
		.map(|(_, reached, release)| (Arc::clone(reached), Arc::clone(release)));

	if let Some((reached, release)) = hook {
		reached.notify_one();
		release.notified().await;
	}
}

#[cfg(not(debug_assertions))]
pub(super) async fn maybe_pause_after_hot_stage(_database_branch_id: DatabaseBranchId) {}

#[cfg(debug_assertions)]
impl Drop for PauseGuard {
	fn drop(&mut self) {
		*self.slot.lock() = None;
	}
}
