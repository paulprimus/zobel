use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use tokio::time::Instant;

mod clamav;

#[derive(Parser)]
#[command(name = "gs")]
#[command(author, version, about = "ClamAV Stream Client", long_about = None,)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Ping,
    Scan { filename: String },
    Stats,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("debug,tower=trace")
        .init();
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Ping) => {
            clamav::ping()
                .await
                .context("Ping konnte nicht durchgefuehrt werden!")?;
        }
        Some(Commands::Scan { filename }) => {
            let stopwatch = Instant::now();
            tracing::info!("Scanning {}", filename);
            clamav::instream(filename.to_owned())
                .await
                .context("Dokument wurde nicht gescannt!")?;
            let duration = stopwatch.elapsed();
            tracing::info!("Duration: {:?}", duration);
        }
        Some(Commands::Stats) => {
            clamav::stats()
                .await
                .context("Stats konnten nicht abgerufen werden")?;
        }
        None => {}
    };
    Ok(())
}
