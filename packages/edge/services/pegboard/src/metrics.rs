use rivet_metrics::{prometheus::*, BUCKETS, MICRO_BUCKETS, REGISTRY};

lazy_static::lazy_static! {
	pub static ref CLIENT_DUPLICATE_EVENT: IntCounterVec = register_int_counter_vec_with_registry!(
		"pegboard_client_duplicate_event",
		"Duplicate client event that was attempted to be inserted.",
		&["client_id", "index"],
		*REGISTRY
	).unwrap();

	pub static ref CLIENT_MEMORY_TOTAL: IntGaugeVec = register_int_gauge_vec_with_registry!(
		"pegboard_client_memory_total",
		"Total MiB of memory available on a client.",
		&["client_id", "flavor", "state"],
		*REGISTRY
	).unwrap();

	pub static ref CLIENT_CPU_TOTAL: IntGaugeVec = register_int_gauge_vec_with_registry!(
		"pegboard_client_cpu_total",
		"Total millicores of cpu available on a client.",
		&["client_id", "flavor", "state"],
		*REGISTRY
	).unwrap();

	pub static ref CLIENT_MEMORY_ALLOCATED: IntGaugeVec = register_int_gauge_vec_with_registry!(
		"pegboard_client_memory_allocated",
		"Total MiB of memory allocated on a client.",
		&["client_id", "flavor", "state"],
		*REGISTRY
	).unwrap();

	pub static ref CLIENT_CPU_ALLOCATED: IntGaugeVec = register_int_gauge_vec_with_registry!(
		"pegboard_client_cpu_allocated",
		"Total millicores of cpu allocated on a client.",
		&["client_id", "flavor", "state"],
		*REGISTRY
	).unwrap();

	pub static ref ACTOR_ALLOCATE_DURATION: HistogramVec = register_histogram_vec_with_registry!(
		"pegboard_actor_allocate_duration",
		"Total duration to reserve resources for an actor.",
		&["did_reserve"],
		MICRO_BUCKETS.to_vec(),
		*REGISTRY,
	).unwrap();

	pub static ref ACTOR_START_DURATION: HistogramVec = register_histogram_vec_with_registry!(
		"pegboard_actor_start_duration",
		"Total duration from actor creation to starting state.",
		&[],
		BUCKETS.to_vec(),
		*REGISTRY,
	).unwrap();

	pub static ref ENV_MEMORY_USAGE: IntGaugeVec = register_int_gauge_vec_with_registry!(
		"pegboard_env_memory_usage",
		"Total MiB of memory used by an environment.",
		&["env_id", "flavor"],
		*REGISTRY,
	).unwrap();

	pub static ref ENV_CPU_USAGE: IntGaugeVec = register_int_gauge_vec_with_registry!(
		"pegboard_env_cpu_usage",
		"Total millicores used by an environment.",
		&["env_id", "flavor"],
		*REGISTRY,
	).unwrap();
}
