mod config;
mod error;

use crate::config::Config;

use std::path::PathBuf;

use anyhow::{anyhow, Result};
use clap::{Args, Parser, Subcommand};

use tracing::subscriber::set_global_default;
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    name: Option<String>,

    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short, long)]
    debug: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Test(TestArgs),
}

#[derive(Args)]
struct TestArgs {
    #[arg(short, long)]
    data: String
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set logging level
    let filter: String;
    if cli.debug {
        filter = String::from("debug");
    } else {
        filter = String::from("info");
    }

    // Initialize all logging for the head node
    LogTracer::init().expect("Failed to set logger");

    // Default to info log level
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(filter));

    // Write to stdout by default
    let formatting_layer = tracing_subscriber::fmt::layer().with_writer(std::io::stdout);
    let subscriber = Registry::default().with(env_filter).with(formatting_layer);
    set_global_default(subscriber).expect("Failed to set subscriber");

    // Read Config File
    let config: error::Result<Config>;

    if let Some(c) = cli.config {
        config = Config::new(Some(c));
    } else {
        config = Config::new(None);
    }

    if config.is_err() {
        let err = config.err().unwrap();
        tracing::error!("Error: {:?}", err);
        return Err(anyhow!("Error: {:?}", err));
    }

    // Main app

    Ok(())
}
