use std::convert::{TryFrom, TryInto};

use chirp_workflow::prelude::*;
use cluster::{
	metrics,
	types::{Datacenter, PoolType},
};

pub async fn start(config: rivet_config::Config, pools: rivet_pools::Pools) -> GlobalResult<()> {
	let mut interval = tokio::time::interval(std::time::Duration::from_secs(7));
	loop {
		interval.tick().await;

		run_from_env(config.clone(), pools.clone()).await?;
	}
}

#[derive(sqlx::FromRow)]
struct ServerRow {
	datacenter_id: Uuid,
	pool_type: i64,
	is_provisioned: bool,
	is_installed: bool,
	has_nomad_node: bool,
	has_pegboard_client: bool,
	is_draining: bool,
	is_drained: bool,
	is_tainted: bool,
}

#[derive(Debug)]
struct Server {
	datacenter_id: Uuid,
	pool_type: PoolType,
	is_provisioned: bool,
	is_installed: bool,
	has_nomad_node: bool,
	has_pegboard_client: bool,
	is_draining: bool,
	is_tainted: bool,
}

impl TryFrom<ServerRow> for Server {
	type Error = GlobalError;

	fn try_from(value: ServerRow) -> GlobalResult<Self> {
		Ok(Server {
			datacenter_id: value.datacenter_id,
			pool_type: unwrap!(PoolType::from_repr(value.pool_type.try_into()?)),
			is_provisioned: value.is_provisioned,
			is_installed: value.is_installed,
			has_nomad_node: value.has_nomad_node,
			has_pegboard_client: value.has_pegboard_client,
			is_tainted: value.is_tainted,
			is_draining: value.is_draining && !value.is_drained,
		})
	}
}

#[tracing::instrument(skip_all)]
pub async fn run_from_env(
	config: rivet_config::Config,
	pools: rivet_pools::Pools,
) -> GlobalResult<()> {
	let client =
		chirp_client::SharedClient::from_env(pools.clone())?.wrap_new("cluster-metrics-publish");
	let cache = rivet_cache::CacheInner::from_env(&config, pools.clone())?;
	let ctx = StandaloneCtx::new(
		db::DatabaseCrdbNats::from_pools(pools.clone())?,
		config,
		rivet_connection::Connection::new(client, pools, cache),
		"cluster-metrics-publish",
	)
	.await?;

	let servers = select_servers(&ctx).await?;

	let datacenters_res = ctx
		.op(cluster::ops::datacenter::get::Input {
			datacenter_ids: servers.iter().map(|s| s.datacenter_id).collect::<Vec<_>>(),
		})
		.await?;

	for dc in &datacenters_res.datacenters {
		insert_metrics(dc, &servers)?;
	}

	Ok(())
}

async fn select_servers(ctx: &StandaloneCtx) -> GlobalResult<Vec<Server>> {
	let servers = sql_fetch_all!(
		[ctx, ServerRow]
		"
		SELECT
			datacenter_id, pool_type,
			(provider_server_id IS NOT NULL) AS is_provisioned,
			(install_complete_ts IS NOT NULL) AS is_installed,
			(nomad_node_id IS NOT NULL) AS has_nomad_node,
			(pegboard_client_id IS NOT NULL) AS has_pegboard_client,
			(drain_ts IS NOT NULL) AS is_draining,
			(drain_complete_ts IS NOT NULL) AS is_drained,
			(taint_ts IS NOT NULL) AS is_tainted
		FROM db_cluster.servers AS OF SYSTEM TIME '-1s'
		WHERE
			-- Filters out servers that are being destroyed/already destroyed
			cloud_destroy_ts IS NULL
		",
	)
	.await?;

	servers
		.into_iter()
		.map(TryInto::try_into)
		.collect::<GlobalResult<Vec<_>>>()
}

fn insert_metrics(dc: &Datacenter, servers: &[Server]) -> GlobalResult<()> {
	let servers_in_dc = servers
		.iter()
		.filter(|s| s.datacenter_id == dc.datacenter_id);

	let datacenter_id = dc.datacenter_id.to_string();
	let cluster_id = dc.cluster_id.to_string();

	let servers_per_pool = [
		PoolType::Job,
		PoolType::Gg,
		PoolType::Ats,
		PoolType::Pegboard,
		PoolType::PegboardIsolate,
		PoolType::Fdb,
		PoolType::Worker,
		PoolType::Nats,
		PoolType::Guard,
	]
	.into_iter()
	.map(|pool_type| {
		(
			pool_type,
			servers_in_dc
				.clone()
				.filter(|s| s.pool_type == pool_type)
				.collect::<Vec<_>>(),
		)
	})
	.collect::<Vec<_>>();

	// Aggregate all states per pool type
	for (pool_type, servers) in servers_per_pool {
		let mut provisioning = 0;
		let mut installing = 0;
		let mut active = 0;
		let mut nomad = 0;
		let mut pegboard = 0;
		let mut draining = 0;
		let mut tainted = 0;
		let mut draining_tainted = 0;

		for server in servers {
			if server.is_draining {
				draining += 1;
			} else if server.is_provisioned {
				if server.is_installed {
					active += 1;

					if server.has_nomad_node {
						nomad += 1;
					}

					if server.has_pegboard_client {
						pegboard += 1;
					}
				} else {
					installing += 1;
				}
			} else {
				provisioning += 1;
			}

			if server.is_tainted {
				tainted += 1;

				if server.is_draining {
					draining_tainted += 1;
				}
			}
		}

		let labels = [
			cluster_id.as_str(),
			datacenter_id.as_str(),
			&dc.name_id,
			&pool_type.to_string(),
		];

		metrics::PROVISIONING_SERVERS
			.with_label_values(&labels)
			.set(provisioning);
		metrics::INSTALLING_SERVERS
			.with_label_values(&labels)
			.set(installing);
		metrics::ACTIVE_SERVERS
			.with_label_values(&labels)
			.set(active);
		metrics::DRAINING_SERVERS
			.with_label_values(&labels)
			.set(draining);
		metrics::TAINTED_SERVERS
			.with_label_values(&labels)
			.set(tainted);
		metrics::DRAINING_TAINTED_SERVERS
			.with_label_values(&labels)
			.set(draining_tainted);

		match pool_type {
			PoolType::Job => {
				metrics::NOMAD_SERVERS
					.with_label_values(&[&cluster_id, &datacenter_id, &dc.name_id])
					.set(nomad);
			}
			PoolType::Pegboard => {
				metrics::PEGBOARD_SERVERS
					.with_label_values(&[&cluster_id, &datacenter_id, &dc.name_id])
					.set(pegboard);
			}
			PoolType::PegboardIsolate => {
				metrics::PEGBOARD_ISOLATE_SERVERS
					.with_label_values(&[&cluster_id, &datacenter_id, &dc.name_id])
					.set(pegboard);
			}
			_ => {}
		}
	}

	Ok(())
}
