//! Thunder Service - In-Network Post Retrieval
//!
//! This is a simplified entrypoint for the Thunder service,
//! demonstrating the architecture of X's in-network post storage.

use anyhow::Result;
use clap::Parser;
use log::info;

use thunder::args;
use thunder::candidate_source::InMemoryCandidateSource;
use thunder::config::ThunderConfig;
use thunder::realtime_query::{execute_query, RealtimeQuery};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let args = args::Args::parse();

    info!(
        "Thunder Service starting (retention: {} seconds / {:.1} days)",
        args.post_retention_seconds,
        args.post_retention_seconds as f64 / 86400.0
    );

    // Initialize in-memory candidate source
    let source = InMemoryCandidateSource::new();
    let config = ThunderConfig {
        max_posts: args.result_limit,
        retention_seconds: args.post_retention_seconds,
    };

    info!("Thunder config: {:?}", config);

    // Example query demonstration
    let query = RealtimeQuery::new(1, vec![100, 200, 300])
        .with_limit(50)
        .with_max_age(7 * 24 * 60 * 60);

    let response = execute_query(&source, &query, &config);
    info!(
        "Example query: {} candidates in {}ms",
        response.candidates.len(),
        response.query_time_ms
    );

    if args.is_serving {
        info!("Starting gRPC server on port {}...", args.grpc_port);
        // In a full implementation, this would start the gRPC service
        // For now, we just log the configuration
        info!("Thunder service configured for gRPC on 0.0.0.0:{}", args.grpc_port);
        
        // Keep the service running
        tokio::signal::ctrl_c().await?;
        info!("Received shutdown signal");
    }

    info!("Thunder service terminated");
    Ok(())
}
