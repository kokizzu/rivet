use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use rivetkit_sqlite::database::{NativeDatabaseHandle, open_database_from_engine};
use rivetkit_sqlite::types::{BindParam, ColumnValue, ExecuteRoute};
use sqlite_storage::engine::SqliteEngine;
use sqlite_storage::open::OpenConfig;
use tempfile::TempDir;
use universaldb::Subspace;
use universaldb::driver::RocksDbDatabaseDriver;

struct RemoteDbHarness {
	_db_dir: TempDir,
	engine: Arc<SqliteEngine>,
	actor_id: String,
	generation: u64,
	db: NativeDatabaseHandle,
}

impl RemoteDbHarness {
	async fn open(prefix: &str, now_ms: i64) -> Result<Self> {
		let actor_id = unique_actor_id(prefix);
		let db_dir = tempfile::tempdir()?;
		let driver = RocksDbDatabaseDriver::new(db_dir.path().to_path_buf()).await?;
		let db = universaldb::Database::new(Arc::new(driver));
		let (engine, _compaction_rx) =
			SqliteEngine::new(db, Subspace::new(&(prefix, &actor_id)));
		let engine = Arc::new(engine);
		let opened = engine.open(&actor_id, OpenConfig::new(now_ms)).await?;
		let db = open_database_from_engine(
			Arc::clone(&engine),
			actor_id.clone(),
			opened.generation,
			tokio::runtime::Handle::current(),
			None,
		)
		.await?;

		Ok(Self {
			_db_dir: db_dir,
			engine,
			actor_id,
			generation: opened.generation,
			db,
		})
	}

	async fn reopen(&mut self, now_ms: i64) -> Result<()> {
		self.db.close().await?;
		self.engine.close(&self.actor_id, self.generation).await?;
		let opened = self.engine.open(&self.actor_id, OpenConfig::new(now_ms)).await?;
		self.generation = opened.generation;
		self.db = open_database_from_engine(
			Arc::clone(&self.engine),
			self.actor_id.clone(),
			opened.generation,
			tokio::runtime::Handle::current(),
			None,
		)
		.await?;
		Ok(())
	}

	async fn close(self) -> Result<()> {
		self.db.close().await?;
		self.engine.close(&self.actor_id, self.generation).await?;
		Ok(())
	}
}

#[tokio::test]
async fn remote_migration_order_persists_across_reopen() -> Result<()> {
	let mut harness = RemoteDbHarness::open("remote-migration-order", 1).await?;

	harness
		.db
		.execute_write(
			"CREATE TABLE __rivet_migrations(id INTEGER PRIMARY KEY, name TEXT NOT NULL);"
				.to_string(),
			None,
		)
		.await?;
	harness
		.db
		.execute_write(
			"CREATE TABLE items(id INTEGER PRIMARY KEY, value TEXT NOT NULL);".to_string(),
			None,
		)
		.await?;
	harness
		.db
		.execute_write(
			"INSERT INTO __rivet_migrations(id, name) VALUES (?, ?);".to_string(),
			Some(vec![
				BindParam::Integer(1),
				BindParam::Text("create-items".to_string()),
			]),
		)
		.await?;

	let before_reopen = harness
		.db
		.execute(
			"SELECT name FROM __rivet_migrations ORDER BY id;".to_string(),
			None,
		)
		.await?;
	assert_eq!(
		before_reopen.rows,
		vec![vec![ColumnValue::Text("create-items".to_string())]]
	);

	harness.reopen(2).await?;
	let after_reopen = harness
		.db
		.execute(
			"SELECT name FROM __rivet_migrations ORDER BY id;".to_string(),
			None,
		)
		.await?;
	assert_eq!(
		after_reopen.rows,
		vec![vec![ColumnValue::Text("create-items".to_string())]]
	);

	let table_check = harness
		.db
		.execute(
			"SELECT name FROM sqlite_master WHERE type = 'table' AND name = 'items';"
				.to_string(),
			None,
		)
		.await?;
	assert_eq!(
		table_check.rows,
		vec![vec![ColumnValue::Text("items".to_string())]]
	);

	harness.close().await
}

#[tokio::test]
async fn remote_execute_write_forces_writer_for_readonly_sql() -> Result<()> {
	let harness = RemoteDbHarness::open("remote-execute-write", 1).await?;
	harness
		.db
		.execute_write(
			"CREATE TABLE force_writer(id INTEGER PRIMARY KEY);".to_string(),
			None,
		)
		.await?;

	let result = harness
		.db
		.execute_write("SELECT COUNT(*) FROM force_writer;".to_string(), None)
		.await?;

	assert_eq!(result.route, ExecuteRoute::Write);
	assert_eq!(result.rows, vec![vec![ColumnValue::Integer(0)]]);

	harness.close().await
}

#[tokio::test]
async fn remote_manual_transactions_stay_on_writer_until_commit_or_rollback() -> Result<()> {
	let harness = RemoteDbHarness::open("remote-manual-transactions", 1).await?;
	harness
		.db
		.execute_write(
			"CREATE TABLE tx_items(id INTEGER PRIMARY KEY, value TEXT NOT NULL);".to_string(),
			None,
		)
		.await?;

	harness.db.execute("BEGIN".to_string(), None).await?;
	harness
		.db
		.execute(
			"INSERT INTO tx_items(id, value) VALUES (1, 'committed');".to_string(),
			None,
		)
		.await?;
	let in_tx_read = harness
		.db
		.execute(
			"SELECT COUNT(*) FROM tx_items WHERE value = 'committed';".to_string(),
			None,
		)
		.await?;
	assert_eq!(in_tx_read.route, ExecuteRoute::WriteFallback);
	assert_eq!(in_tx_read.rows, vec![vec![ColumnValue::Integer(1)]]);
	harness.db.execute("COMMIT".to_string(), None).await?;

	harness.db.execute("BEGIN".to_string(), None).await?;
	harness
		.db
		.execute(
			"INSERT INTO tx_items(id, value) VALUES (2, 'rolled-back');".to_string(),
			None,
		)
		.await?;
	harness.db.execute("ROLLBACK".to_string(), None).await?;

	harness.db.execute("BEGIN".to_string(), None).await?;
	harness
		.db
		.execute(
			"INSERT INTO tx_items(id, value) VALUES (3, 'savepoint-base');".to_string(),
			None,
		)
		.await?;
	harness
		.db
		.execute("SAVEPOINT patch".to_string(), None)
		.await?;
	harness
		.db
		.execute(
			"UPDATE tx_items SET value = 'patched' WHERE id = 3;".to_string(),
			None,
		)
		.await?;
	harness
		.db
		.execute("ROLLBACK TO patch".to_string(), None)
		.await?;
	harness
		.db
		.execute("RELEASE patch".to_string(), None)
		.await?;
	harness.db.execute("COMMIT".to_string(), None).await?;

	let rows = harness
		.db
		.execute("SELECT id, value FROM tx_items ORDER BY id;".to_string(), None)
		.await?;
	assert_eq!(
		rows.rows,
		vec![
			vec![
				ColumnValue::Integer(1),
				ColumnValue::Text("committed".to_string())
			],
			vec![
				ColumnValue::Integer(3),
				ColumnValue::Text("savepoint-base".to_string())
			],
		]
	);

	harness.close().await
}

fn unique_actor_id(prefix: &str) -> String {
	let nanos = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.expect("system time should be after epoch")
		.as_nanos();
	format!("{prefix}-{nanos}")
}
