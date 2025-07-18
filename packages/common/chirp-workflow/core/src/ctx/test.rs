use global_error::{GlobalError, GlobalResult};
use rivet_config::{
	config::{Addresses, FoundationDb, Redis, RedisTypes, Root},
	secret::Secret,
};
use rivet_pools::prelude::*;
use serde::Serialize;
use tracing::Instrument;
use url::Url;
use uuid::Uuid;

use crate::{
	builder::{common as builder, WorkflowRepr},
	ctx::{
		common,
		message::{SubscriptionHandle, TailAnchor, TailAnchorResponse},
		MessageCtx,
	},
	db::{Database, DatabaseHandle},
	message::{Message, NatsMessage},
	operation::{Operation, OperationInput},
	signal::Signal,
	utils::{self, tags::AsTags},
	workflow::{Workflow, WorkflowInput},
};

#[derive(Clone)]
pub struct TestCtx {
	name: String,
	ray_id: Uuid,
	ts: i64,

	db: DatabaseHandle,

	config: rivet_config::Config,
	conn: rivet_connection::Connection,
	msg_ctx: MessageCtx,

	// Backwards compatibility
	op_ctx: rivet_operation::OperationContext<()>,
}

impl TestCtx {
	#[tracing::instrument]
	pub async fn from_env<DB: Database + Sync + 'static>(
		test_name: &str,
		no_config: bool,
	) -> TestCtx {
		let service_name = format!("{}-test--{}", rivet_env::service_name(), test_name);

		let ray_id = Uuid::new_v4();
		let (config, pools) = if no_config {
			let mut root = Root::default();
			root.server.as_mut().unwrap().foundationdb = Some(FoundationDb {
				cluster_description: "docker".to_string(),
				cluster_id: "docker".to_string(),
				addresses: Addresses::Static(vec!["127.0.0.1:4500".to_string()]),
				port: None,
			});
			root.server.as_mut().unwrap().redis = RedisTypes {
				ephemeral: Redis {
					url: Url::parse("redis://127.0.0.1:6379").unwrap(),
					username: None,
					password: Some(Secret::new("password".to_string())),
				},
				persistent: Redis {
					url: Url::parse("redis://127.0.0.1:6379").unwrap(),
					username: None,
					password: Some(Secret::new("password".to_string())),
				},
			};
			let config = rivet_config::Config::from_root(root);
			let pools = rivet_pools::Pools::test(config.clone())
				.await
				.expect("failed to create pools");

			(config, pools)
		} else {
			let config = rivet_config::Config::load::<String>(&[]).await.unwrap();
			let pools = rivet_pools::Pools::new(config.clone())
				.await
				.expect("failed to create pools");

			(config, pools)
		};
		let shared_client = chirp_client::SharedClient::from_env(pools.clone())
			.expect("failed to create chirp client");
		let cache = rivet_cache::CacheInner::from_env(&config, pools.clone())
			.expect("failed to create cache");
		let conn = utils::new_conn(
			&shared_client,
			&pools,
			&cache,
			ray_id,
			Uuid::new_v4(),
			&service_name,
		);
		let ts = rivet_util::timestamp::now();
		let req_id = Uuid::new_v4();
		let op_ctx = rivet_operation::OperationContext::new(
			service_name.to_string(),
			std::time::Duration::from_secs(60),
			config.clone(),
			conn.clone(),
			req_id,
			ray_id,
			ts,
			ts,
			(),
		);

		let db = DB::from_pools(pools).unwrap();
		let msg_ctx = MessageCtx::new(&conn, ray_id).await.unwrap();

		TestCtx {
			name: service_name,
			ray_id,
			ts: rivet_util::timestamp::now(),
			db,
			config,
			conn,
			op_ctx,
			msg_ctx,
		}
	}
}

impl TestCtx {
	/// Creates a workflow builder.
	pub fn workflow<I>(
		&self,
		input: impl WorkflowRepr<I>,
	) -> builder::workflow::WorkflowBuilder<impl WorkflowRepr<I>, I>
	where
		I: WorkflowInput,
		<I as WorkflowInput>::Workflow: Workflow<Input = I>,
	{
		builder::workflow::WorkflowBuilder::new(self.db.clone(), self.ray_id, input, false)
	}

	/// Creates a signal builder.
	pub fn signal<T: Signal + Serialize>(&self, body: T) -> builder::signal::SignalBuilder<T> {
		builder::signal::SignalBuilder::new(self.db.clone(), self.ray_id, body, false)
	}

	#[tracing::instrument(skip_all, fields(operation_name=I::Operation::NAME))]
	pub async fn op<I>(
		&self,
		input: I,
	) -> GlobalResult<<<I as OperationInput>::Operation as Operation>::Output>
	where
		I: OperationInput,
		<I as OperationInput>::Operation: Operation<Input = I>,
	{
		common::op(
			&self.db,
			&self.config,
			&self.conn,
			self.ray_id,
			self.ts,
			false,
			input,
		)
		.in_current_span()
		.await
	}

	pub fn msg<M: Message>(&self, body: M) -> builder::message::MessageBuilder<M> {
		builder::message::MessageBuilder::new(self.msg_ctx.clone(), body)
	}

	#[tracing::instrument(skip_all, fields(message=M::NAME))]
	pub async fn subscribe<M>(&self, tags: impl AsTags) -> GlobalResult<SubscriptionHandle<M>>
	where
		M: Message,
	{
		self.msg_ctx
			.subscribe::<M>(tags)
			.in_current_span()
			.await
			.map_err(GlobalError::raw)
	}

	#[tracing::instrument(skip_all, fields(message=M::NAME))]
	pub async fn tail_read<M>(&self, tags: impl AsTags) -> GlobalResult<Option<NatsMessage<M>>>
	where
		M: Message,
	{
		self.msg_ctx
			.tail_read::<M>(tags)
			.in_current_span()
			.await
			.map_err(GlobalError::raw)
	}

	#[tracing::instrument(skip_all, fields(message=M::NAME))]
	pub async fn tail_anchor<M>(
		&self,
		tags: impl AsTags,
		anchor: &TailAnchor,
	) -> GlobalResult<TailAnchorResponse<M>>
	where
		M: Message,
	{
		self.msg_ctx
			.tail_anchor::<M>(tags, anchor)
			.in_current_span()
			.await
			.map_err(GlobalError::raw)
	}
}

impl TestCtx {
	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn req_id(&self) -> Uuid {
		self.op_ctx.req_id()
	}

	pub fn ray_id(&self) -> Uuid {
		self.ray_id
	}

	/// Timestamp at which the request started.
	pub fn ts(&self) -> i64 {
		self.ts
	}

	/// Timestamp at which the request was published.
	pub fn req_ts(&self) -> i64 {
		self.op_ctx.req_ts()
	}

	/// Time between when the timestamp was processed and when it was published.
	pub fn req_dt(&self) -> i64 {
		self.ts.saturating_sub(self.op_ctx.req_ts())
	}

	pub fn config(&self) -> &rivet_config::Config {
		&self.config
	}

	pub fn trace(&self) -> &[chirp_client::TraceEntry] {
		self.conn.trace()
	}

	pub fn chirp(&self) -> &chirp_client::Client {
		self.conn.chirp()
	}

	pub fn conn(&self) -> &rivet_connection::Connection {
		&self.conn
	}

	pub fn cache(&self) -> rivet_cache::RequestConfig {
		self.conn.cache()
	}

	pub fn cache_handle(&self) -> rivet_cache::Cache {
		self.conn.cache_handle()
	}

	pub fn pools(&self) -> &rivet_pools::Pools {
		self.conn.pools()
	}

	#[tracing::instrument(skip_all)]
	pub async fn crdb(&self) -> Result<CrdbPool, rivet_pools::Error> {
		self.conn.crdb().await
	}

	#[tracing::instrument(skip_all)]
	pub async fn redis_cache(&self) -> Result<RedisPool, rivet_pools::Error> {
		self.conn.redis_cache().await
	}

	#[tracing::instrument(skip_all)]
	pub async fn redis_cdn(&self) -> Result<RedisPool, rivet_pools::Error> {
		self.conn.redis_cdn().await
	}

	#[tracing::instrument(skip_all)]
	pub async fn redis_job(&self) -> Result<RedisPool, rivet_pools::Error> {
		self.conn.redis_job().await
	}

	#[tracing::instrument(skip_all)]
	pub async fn redis_mm(&self) -> Result<RedisPool, rivet_pools::Error> {
		self.conn.redis_mm().await
	}

	#[tracing::instrument(skip_all)]
	pub async fn clickhouse(&self) -> GlobalResult<ClickHousePool> {
		self.conn.clickhouse().await
	}

	#[tracing::instrument(skip_all)]
	pub async fn clickhouse_inserter(&self) -> GlobalResult<ClickHouseInserterHandle> {
		self.conn.clickhouse_inserter().await
	}

	#[tracing::instrument(skip_all, fields(%workflow_id))]
	pub async fn sqlite_for_workflow(&self, workflow_id: Uuid) -> GlobalResult<SqlitePool> {
		common::sqlite_for_workflow(&self.db, &self.conn, workflow_id, true)
			.in_current_span()
			.await
	}

	// Backwards compatibility
	pub fn op_ctx(&self) -> &rivet_operation::OperationContext<()> {
		&self.op_ctx
	}
}
