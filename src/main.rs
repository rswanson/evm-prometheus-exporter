use alloy_rpc_client::RpcClient;
use alloy_transport_http::Http;
use anyhow::Result;
use clap::Parser;
use metrics::{describe_gauge, gauge};
use metrics_exporter_prometheus::PrometheusBuilder;
use std::sync::Arc;
use tokio::time::{self, Duration};

/// CLI arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// RPC endpoint URL
    #[arg(long, default_value = "http://localhost:8545")]
    rpc_url: String,

    /// Metrics server port
    #[arg(long, default_value = "9184")]
    metrics_port: u16,

    /// Scrape interval in seconds
    #[arg(long, default_value = "15")]
    scrape_interval: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Parse command line arguments
    let args = Args::parse();

    // Create HTTP transport and RPC client
    let transport = Http::new(&args.rpc_url)?;
    let client = Arc::new(RpcClient::new(transport));

    // Initialize Prometheus exporter
    let builder = PrometheusBuilder::new();
    builder
        .with_http_listener(([0, 0, 0, 0], args.metrics_port))
        .install()?;

    // Describe our metrics
    describe_gauge!(
        "evm_block_height",
        "Current block height of the chain"
    );

    // Metrics collection loop
    let mut interval = time::interval(Duration::from_secs(args.scrape_interval));
    loop {
        interval.tick().await;
        if let Err(e) = update_metrics(client.clone()).await {
            tracing::error!("Failed to update metrics: {}", e);
        }
    }
}

async fn update_metrics(client: Arc<RpcClient<Http>>) -> Result<()> {
    // Get the latest block number
    let block_number = client.get_block_number().await?;
    
    // Update the block height metric
    gauge!("evm_block_height", block_number.to_f64());

    Ok(())
} 