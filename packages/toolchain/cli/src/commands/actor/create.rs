use anyhow::*;
use clap::{Parser, ValueEnum};
use serde::Deserialize;
use std::collections::HashMap;
use toolchain::{
	build, errors,
	rivet_api::{apis, models},
};
use uuid::Uuid;

#[derive(ValueEnum, Clone)]
enum NetworkMode {
	Bridge,
	Host,
}

/// Custom struct that includes the port name in it. The name is mapped to the key in the `ports`
/// map.
#[derive(Deserialize)]
struct Port {
	name: String,
	protocol: models::ActorsPortProtocol,
	internal_port: Option<i32>,
	// Temporarily disabled
	// guard: Option<models::ActorsGuardRouting>,
	#[serde(default)]
	guard: bool,
	#[serde(default)]
	host: bool,
}

/// Create a new actor
#[derive(Parser)]
pub struct Opts {
	/// Specify the environment to create the actor in (will prompt if not specified)
	#[clap(long, alias = "env", short = 'e')]
	environment: Option<String>,

	/// Specify the region to create the actor in (will auto-select if not specified)
	#[clap(long, short = 'r')]
	region: Option<String>,

	/// Tags to use for both the actor & build tags. This allows for creating actors quickly since
	/// the tags are often identical between the two.
	#[clap(long = "tags", short = 't')]
	universal_tags: Option<String>,

	/// Tags specific to the actor (key=value format)
	#[clap(long, short = 'a')]
	actor_tags: Option<String>,

	/// Specific build ID to use for this actor
	#[clap(long)]
	build: Option<String>,

	/// Tags to identify the build to use (key=value format)
	#[clap(long, short = 'b')]
	build_tags: Option<String>,

	/// Override the automatically generated version name
	#[clap(
		long,
		short = 'v',
		help = "Override the automatically generated version name"
	)]
	version: Option<String>,

	/// Environment variables to pass to the actor (key=value format)
	#[clap(long = "env-var")]
	env_vars: Option<Vec<String>>,

	/// Network mode for the actor: bridge (default, isolated network) or host (shares host network stack)
	#[clap(long, value_enum)]
	network_mode: Option<NetworkMode>,

	/// Ports to expose from the actor (name=protocol:port format)
	#[clap(long = "port", short = 'p')]
	ports: Option<Vec<String>>,

	/// CPU resource limit for the actor (in mCPU)
	#[clap(long)]
	cpu: Option<i32>,

	/// Memory resource limit for the actor (in MiB)
	#[clap(long)]
	memory: Option<i32>,

	/// Time in seconds to wait before forcefully killing the actor
	#[clap(long)]
	kill_timeout: Option<i64>,

	/// Create a durable actor that persists until explicitly destroyed
	#[clap(long)]
	durable: bool,

	/// If included, the `current` tag will not be automatically inserted to the build tag
	#[clap(long)]
	no_build_current_tag: bool,

	/// Stream logs after the actor is created
	#[clap(long)]
	logs: bool,

	/// Specify which log stream to display
	#[clap(long)]
	log_stream: Option<toolchain::util::actor::logs::LogStream>,

	/// Deploy the build before creating the actor
	#[clap(long)]
	deploy: bool,
}

impl Opts {
	pub async fn execute(&self) -> Result<()> {
		let ctx = crate::util::login::load_or_login().await?;

		let env = crate::util::env::get_or_select(&ctx, self.environment.as_ref()).await?;

		// Parse tags
		let actor_tags = if let Some(t) = &self.actor_tags {
			kv_str::from_str::<HashMap<String, String>>(t)?
		} else if let Some(t) = &self.universal_tags {
			kv_str::from_str::<HashMap<String, String>>(t)?
		} else {
			// No tags
			HashMap::new()
		};

		// Parse build ID
		let mut build_id = self
			.build
			.as_ref()
			.map(|b| Uuid::parse_str(&b))
			.transpose()
			.context("invalid build uuid")?;

		// Parse build tags
		let mut build_tags = if let Some(t) = &self.build_tags {
			Some(kv_str::from_str::<HashMap<String, String>>(t)?)
		} else if let Some(t) = &self.universal_tags {
			Some(kv_str::from_str::<HashMap<String, String>>(t)?)
		} else {
			None
		};

		// Parse ports
		let ports = self
			.ports
			.as_ref()
			.map(|ports| {
				ports
					.iter()
					.map(|port_str| {
						let port = kv_str::from_str::<Port>(port_str)?;
						Ok((
							port.name,
							models::ActorsCreateActorPortRequest {
								internal_port: port.internal_port,
								protocol: port.protocol,
								routing: Some(Box::new(models::ActorsPortRouting {
									// Temporarily disabled
									// guard: port.guard.map(Box::new),
									guard: port.guard.then_some(serde_json::json!({})),
									host: port.host.then_some(serde_json::json!({})),
								})),
							},
						))
					})
					.collect::<Result<HashMap<String, models::ActorsCreateActorPortRequest>>>()
			})
			.transpose()?;

		// Parse environment variables
		let env_vars = self
			.env_vars
			.as_ref()
			.map(|env_vars| {
				env_vars
					.iter()
					.map(|env| {
						env.split_once('=')
							.map(|(k, v)| (k.to_string(), v.to_string()))
							.with_context(|| anyhow!("invalid env value: {env}"))
					})
					.collect::<Result<HashMap<String, String>>>()
			})
			.transpose()?;

		// Auto-deploy
		if self.deploy {
			// Remove build tags, since we'll be using the build ID
			let build_tags = build_tags
				.take()
				.context("must define build tags when using deploy flag")?;

			// Deploy server
			let deploy_build_ids = crate::util::deploy::deploy(crate::util::deploy::DeployOpts {
				ctx: &ctx,
				environment: &env,
				filter_tags: None,
				build_tags: Some(build_tags),
				version: self.version.clone(),
				skip_route_creation: None,
				keep_existing_routes: None,
				non_interactive: false,
				upgrade: false,
			})
			.await?;

			if deploy_build_ids.len() > 1 {
				println!("Warning: Multiple build IDs match tags, proceeding with first");
			}

			let deploy_build_id = deploy_build_ids
				.first()
				.context("No builds matched build tags")?;
			build_id = Some(*deploy_build_id);
		}

		// Automatically add `current` tag to make querying easier
		//
		// Do this AFTER the deploy since this will mess up the build filter.
		if !self.no_build_current_tag {
			if let Some(build_tags) = build_tags.as_mut() {
				if !build_tags.contains_key(build::tags::VERSION) {
					build_tags.insert(build::tags::CURRENT.into(), "true".into());
				}
			}
		}

		// Auto-select region if needed
		let region = if let Some(region) = &self.region {
			region.clone()
		} else {
			let regions = apis::regions_api::regions_list(
				&ctx.openapi_config_cloud,
				Some(&ctx.project.name_id.to_string()),
				Some(&env),
			)
			.await?;

			// TODO(RVT-4207): Improve automatic region selection logic
			// Choose a region
			let auto_region = if let Some(ideal_region) = regions
				.regions
				.iter()
				.filter(|r| r.id == "lax" || r.id == "local")
				.next()
			{
				ideal_region.id.clone()
			} else {
				regions.regions.first().context("no regions")?.id.clone()
			};
			println!("Automatically selected region: {auto_region}");

			auto_region
		};

		let resources = match (self.cpu, self.memory) {
			(Some(cpu), Some(memory)) => Some(Box::new(models::ActorsResources { cpu, memory })),
			(Some(_), None) | (None, Some(_)) => {
				return Err(errors::UserError::new("Must define both --cpu and --memory").into())
			}
			(None, None) => None,
		};

		let request = models::ActorsCreateActorRequest {
			region: Some(region),
			tags: Some(serde_json::json!(actor_tags)),
			build: build_id,
			build_tags: build_tags.map(|bt| Some(serde_json::json!(bt))),
			runtime: Some(Box::new(models::ActorsCreateActorRuntimeRequest {
				environment: env_vars,
				network: None,
			})),
			network: Some(Box::new(models::ActorsCreateActorNetworkRequest {
				mode: self.network_mode.as_ref().map(|mode| match mode {
					NetworkMode::Bridge => models::ActorsNetworkMode::Bridge,
					NetworkMode::Host => models::ActorsNetworkMode::Host,
				}),
				ports,
				wait_ready: Some(false),
			})),
			resources,
			lifecycle: Some(Box::new(models::ActorsLifecycle {
				durable: Some(self.durable),
				kill_timeout: self.kill_timeout,
			})),
		};

		let response = apis::actors_api::actors_create(
			&ctx.openapi_config_cloud,
			request,
			Some(&ctx.project.name_id),
			Some(&env),
			None,
		)
		.await?;
		println!("Created actor:\n{:#?}", response.actor);

		// Tail logs
		if self.logs {
			toolchain::util::actor::logs::tail(
				&ctx,
				toolchain::util::actor::logs::TailOpts {
					environment: &env,
					actor_id: response.actor.id,
					stream: self
						.log_stream
						.clone()
						.unwrap_or(toolchain::util::actor::logs::LogStream::All),
					follow: true,
					print_type: toolchain::util::actor::logs::PrintType::PrintWithTime,
					exit_on_ctrl_c: true,
				},
			)
			.await?;
		}

		Ok(())
	}
}
