use clap::Parser;

mod claim;
mod cmd;
mod config;

use config::Config;

#[derive(clap::Parser)]
struct Args {
    #[clap(short, long)]
    config: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let config = Config::read_from_file(&args.config)?;
    let claim = claim::ClaimReward::new(config);
    claim.run().await?;
    Ok(())
}
