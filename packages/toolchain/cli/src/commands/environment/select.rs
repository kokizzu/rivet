use anyhow::*;
use clap::Parser;

/// Select the default environment to use
#[derive(Parser)]
pub struct Opts {
	/// Environment name to select (will prompt if not specified)
	environment: Option<String>,
}

impl Opts {
	pub async fn execute(&self) -> Result<()> {
		let ctx = crate::util::login::load_or_login().await?;
		crate::util::env::select(&ctx, true, self.environment.as_deref()).await?;
		Ok(())
	}
}
