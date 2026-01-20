//! Command line arguments for Thunder service

use clap::Parser;

/// Command line arguments for the Thunder in-memory post store service
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Post retention period in seconds
    #[arg(long, default_value = "604800")] // 7 days
    pub post_retention_seconds: u64,

    /// Request timeout in milliseconds
    #[arg(long, default_value = "5000")]
    pub request_timeout_ms: u64,

    /// Maximum concurrent requests
    #[arg(long, default_value = "1000")]
    pub max_concurrent_requests: usize,

    /// gRPC server port
    #[arg(long, default_value = "50051")]
    pub grpc_port: u16,

    /// HTTP server port
    #[arg(long, default_value = "8080")]
    pub http_port: u16,

    /// Enable profiling server
    #[arg(long, default_value = "false")]
    pub enable_profiling: bool,

    /// Number of Kafka consumer threads
    #[arg(long, default_value = "4")]
    pub kafka_num_threads: usize,

    /// Whether to serve requests (vs just consume Kafka)
    #[arg(long, default_value = "true")]
    pub is_serving: bool,
}
