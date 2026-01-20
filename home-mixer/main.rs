//! HomeMixer gRPC Server
//!
//! This is the main entry point for the HomeMixer service.

use anyhow::Result;
use clap::Parser;
use log::{error, info};
use std::net::SocketAddr;

use tonic::codec::CompressionEncoding;
use tonic_reflection::server::Builder;

use home_mixer::proto;
use home_mixer::params;
use home_mixer::server::HomeMixerServer;

#[derive(Parser, Debug)]
#[command(about = "HomeMixer gRPC Server")]
struct Args {
    #[arg(long, default_value = "50051")]
    grpc_port: u16,

    #[arg(long, default_value = "8081")]
    metrics_port: u16,

    #[arg(long, default_value = "60")]
    reload_interval_minutes: u64,

    #[arg(long, default_value = "1000")]
    chunk_size: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    let args = Args::parse();

    info!(
        "Starting HomeMixer server with gRPC port: {}, metrics port: {}, reload interval: {} minutes, chunk size: {}",
        args.grpc_port, args.metrics_port, args.reload_interval_minutes, args.chunk_size,
    );

    // Create the service implementation
    let service = HomeMixerServer::new().await;

    // Build gRPC reflection service
    let reflection_service = Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    // Configure the gRPC service
    let grpc_service = proto::scored_posts_service_server::ScoredPostsServiceServer::new(service)
        .max_decoding_message_size(params::MAX_GRPC_MESSAGE_SIZE)
        .max_encoding_message_size(params::MAX_GRPC_MESSAGE_SIZE)
        .accept_compressed(CompressionEncoding::Gzip)
        .send_compressed(CompressionEncoding::Gzip);

    // Start the gRPC server
    let addr: SocketAddr = format!("0.0.0.0:{}", args.grpc_port).parse()?;
    info!("Starting gRPC server on {}", addr);

    let grpc_handle = tokio::spawn(async move {
        if let Err(e) = tonic::transport::Server::builder()
            .add_service(grpc_service)
            .add_service(reflection_service)
            .serve(addr)
            .await
        {
            error!("gRPC server error: {}", e);
        }
    });

    // Start metrics/health HTTP server
    let metrics_addr: SocketAddr = format!("0.0.0.0:{}", args.metrics_port).parse()?;
    info!("Starting metrics server on {}", metrics_addr);

    let app = axum::Router::new()
        .route("/health", axum::routing::get(|| async { "OK" }))
        .route("/ready", axum::routing::get(|| async { "OK" }));

    let http_handle = tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(metrics_addr).await.unwrap();
        if let Err(e) = axum::serve(listener, app).await {
            error!("HTTP server error: {}", e);
        }
    });

    info!("Server ready");

    // Wait for shutdown
    tokio::select! {
        _ = grpc_handle => {},
        _ = http_handle => {},
    }

    info!("Server shutdown complete");
    Ok(())
}
