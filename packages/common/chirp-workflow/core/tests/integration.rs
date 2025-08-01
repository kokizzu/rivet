use std::time::Duration;

use chirp_workflow::prelude::*;

#[test]
fn fdb_sqlite_nats_driver() {
	// SAFETY: No other threads exist yet
	unsafe {
		std::env::set_var("RUST_LOG", "DEBUG");
		std::env::set_var("RUST_LOG_ANSI_COLOR", "1");
		std::env::set_var("RIVET_OTEL_ENABLED", "1");
		std::env::set_var("RIVET_OTEL_SAMPLER_RATIO", "1");
		std::env::set_var("RIVET_SERVICE_NAME", "test");
		std::env::set_var("RIVET_OTEL_ENDPOINT", "http://127.0.0.1:4317");
	}

	// Build runtime
	chirp_workflow::prelude::__rivet_runtime::run(
		chirp_workflow::prelude::tracing::Instrument::instrument(
			async move {
				tracing::info!("test starting");
				fdb_sqlite_nats_driver_inner().await;
				tracing::info!("test finished");
			},
			chirp_workflow::prelude::tracing::info_span!("fdb_sqlite_nats_driver"),
		),
	);
}

async fn fdb_sqlite_nats_driver_inner() {
	let ctx = chirp_workflow::prelude::TestCtx::from_env::<db::DatabaseFdbSqliteNats>(
		"fdb_sqlite_nats_driver",
		true,
	)
	.await;
	let config = ctx.config().clone();
	let pools = ctx.pools().clone();

	let mut reg = Registry::new();
	reg.register_workflow::<def::Workflow>().unwrap();
	let reg = reg.handle();

	let db = db::DatabaseFdbSqliteNats::from_pools(pools.clone()).unwrap();

	// let workflow_id = Uuid::new_v4();
	// let input = serde_json::value::RawValue::from_string("null".to_string()).unwrap();

	// db.dispatch_workflow(
	// 	Uuid::new_v4(),
	// 	workflow_id,
	// 	"workflow_name",
	// 	Some(&json!({ "bald": "eagle" })),
	// 	&input,
	// 	false,
	// )
	// .await
	// .unwrap();

	let workflow_id = ctx.workflow(def::Input {}).dispatch().await.unwrap();

	let ctx2 = ctx.clone();
	tokio::spawn(async move {
		tokio::time::sleep(Duration::from_millis(110)).await;

		ctx2.signal(def::MySignal {
			test: Uuid::new_v4(),
		})
		.to_workflow_id(workflow_id)
		.send()
		.await
		.unwrap();
	});

	let worker = Worker::new(reg.clone(), db.clone());

	// Start worker
	worker.start(config.clone(), pools.clone()).await.unwrap()
}

mod def {
	use chirp_workflow::prelude::*;

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Input {}

	#[workflow]
	pub async fn test(ctx: &mut WorkflowCtx, input: &Input) -> GlobalResult<()> {
		tracing::info!(w=?ctx.workflow_id(), "hello from workflow");

		// ctx.activity(TestActivityInput {
		// 	foo: "bar".to_string(),
		// })
		// .await?;

		let sig = ctx.listen::<MySignal>().await?;

		tracing::info!(?sig, "signal recv ------------------");

		Ok(())
	}

	#[derive(Debug, Serialize, Deserialize, Hash)]
	struct TestActivityInput {
		foo: String,
	}

	#[activity(TestActivity)]
	async fn test_activity(ctx: &ActivityCtx, input: &TestActivityInput) -> GlobalResult<()> {
		tracing::info!(?input.foo, "hello from activity");

		Ok(())
	}

	#[signal("my_signal")]
	#[derive(Debug)]
	pub struct MySignal {
		pub test: Uuid,
	}
}
