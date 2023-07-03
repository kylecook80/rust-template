{%- if config_enable -%}
mod config;
{%- endif -%}
mod error;
{% if config_enable %}
use crate::config::Config;
{%- endif %}
{%- if cli_enable %}
use std::path::PathBuf;
{%- endif %}
use anyhow::{anyhow, Result};
{%- if cli_enable %}
use clap::{Parser, Subcommand};
{%- endif %}
{%- if logging_enable %}
use tracing::subscriber::set_global_default;
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
{%- endif %}
{% if cli_enable %}
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    name: Option<String>,

    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short, long)]
    debug: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Test {
        #[arg(short, long)]
        list: bool,
    },
}
{% endif %}
#[tokio::main]
async fn main() -> Result<()> {
    {%- if cli_enable %}
    let cli = Cli::parse();
    {%- endif %}
    {% if logging_enable and cli_enable %}
    // Set logging level
    let filter: String;
    if cli.debug {
        filter = String::from("debug");
    } else {
        filter = String::from("info");
    }
    {% elsif logging_enable %}
    // Set logging level
    let filter = String::from("info");
    {%- endif %}
    {%- if logging_enable %}
    // Initialize all logging for the head node
    LogTracer::init().expect("Failed to set logger");

    // Default to info log level
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(filter));

    // Write to stdout by default
    let formatting_layer = tracing_subscriber::fmt::layer().with_writer(std::io::stdout);

    let subscriber = Registry::default().with(env_filter).with(formatting_layer);

    set_global_default(subscriber).expect("Failed to set subscriber");
    {%- endif %}
    {% if config_enable %}
    // Read Config File
    let config: error::Result<Config>;
    {%- endif %}
    {%- if config_enable and cli_enable %}
    if let Some(c) = cli.config {
        config = Config::new(Some(c));
    } else {
        config = Config::new(None);
    }
    {% elsif config_enable %}
    config = Config::new(None);
    {%- endif %}
    {%- if config_enable %}
    if config.is_err() {
        let err = config.err().unwrap();
        tracing::error!("Error: {:?}", err);
        return Err(anyhow!("Error: {:?}", err));
    }

    let config = config.unwrap();
    {%- endif %}
    // Main app

    Ok(())
}
