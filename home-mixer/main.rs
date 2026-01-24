//! HomeMixer Server
//!
//! This is the main entry point for the HomeMixer service.
//! It provides both HTTP and demonstration endpoints for the algorithm.

use anyhow::Result;
use axum::{
    extract::Json,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use clap::Parser;
use log::info;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use home_mixer::params;

#[derive(Parser, Debug)]
#[command(about = "HomeMixer Server - X's For You Algorithm")]
struct Args {
    #[arg(long, default_value = "8080")]
    port: u16,
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: String,
    version: String,
}

#[derive(Debug, Serialize)]
struct WeightsResponse {
    reply: f64,
    profile_click: f64,
    bookmark: f64,
    follow_author: f64,
    quote: f64,
    dm_share: f64,
    like: f64,
    repost: f64,
    video_view: f64,
    not_interested: f64,
    block: f64,
    mute: f64,
    report: f64,
}

#[derive(Debug, Deserialize)]
struct ScoreRequest {
    reply_prob: f64,
    like_prob: f64,
    repost_prob: f64,
    profile_click_prob: f64,
    bookmark_prob: f64,
    #[serde(default)]
    video_view_prob: f64,
    #[serde(default)]
    has_link: bool,
}

#[derive(Debug, Serialize)]
struct ScoreResponse {
    score: f64,
    breakdown: ScoreBreakdown,
    tier: String,
}

#[derive(Debug, Serialize)]
struct ScoreBreakdown {
    reply_contribution: f64,
    profile_click_contribution: f64,
    bookmark_contribution: f64,
    like_contribution: f64,
    repost_contribution: f64,
    video_contribution: f64,
    link_penalty: Option<f64>,
}

async fn health() -> impl IntoResponse {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

async fn get_weights() -> impl IntoResponse {
    Json(WeightsResponse {
        reply: params::REPLY_WEIGHT,
        profile_click: params::PROFILE_CLICK_WEIGHT,
        bookmark: params::BOOKMARK_WEIGHT,
        follow_author: params::FOLLOW_AUTHOR_WEIGHT,
        quote: params::QUOTE_WEIGHT,
        dm_share: params::SHARE_VIA_DM_WEIGHT,
        like: params::FAVORITE_WEIGHT,
        repost: params::RETWEET_WEIGHT,
        video_view: params::VQV_WEIGHT,
        not_interested: params::NOT_INTERESTED_WEIGHT,
        block: params::BLOCK_AUTHOR_WEIGHT,
        mute: params::MUTE_AUTHOR_WEIGHT,
        report: params::REPORT_WEIGHT,
    })
}

async fn calculate_score(Json(req): Json<ScoreRequest>) -> impl IntoResponse {
    // Calculate score using weights
    let reply_contribution = req.reply_prob * params::REPLY_WEIGHT;
    let profile_click_contribution = req.profile_click_prob * params::PROFILE_CLICK_WEIGHT;
    let bookmark_contribution = req.bookmark_prob * params::BOOKMARK_WEIGHT;
    let like_contribution = req.like_prob * params::FAVORITE_WEIGHT;
    let repost_contribution = req.repost_prob * params::RETWEET_WEIGHT;
    let video_contribution = req.video_view_prob * params::VQV_WEIGHT;

    let mut score = reply_contribution
        + profile_click_contribution
        + bookmark_contribution
        + like_contribution
        + repost_contribution
        + video_contribution;

    // Apply link penalty (approximately 90% reduction)
    let link_penalty = if req.has_link {
        let penalty = score * 0.9;
        score -= penalty;
        Some(penalty)
    } else {
        None
    };

    // Determine tier
    let tier = if score >= 30.0 {
        "VIRAL_POTENTIAL"
    } else if score >= 15.0 {
        "GOOD"
    } else if score >= 5.0 {
        "AVERAGE"
    } else {
        "LOW"
    }
    .to_string();

    Json(ScoreResponse {
        score,
        breakdown: ScoreBreakdown {
            reply_contribution,
            profile_click_contribution,
            bookmark_contribution,
            like_contribution,
            repost_contribution,
            video_contribution,
            link_penalty,
        },
        tier,
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();

    info!("Starting HomeMixer server on port {}", args.port);
    info!("Algorithm weights loaded from params.rs");
    info!("  Reply weight: {}", params::REPLY_WEIGHT);
    info!("  Profile click weight: {}", params::PROFILE_CLICK_WEIGHT);
    info!("  Bookmark weight: {}", params::BOOKMARK_WEIGHT);
    info!("  Report weight: {}", params::REPORT_WEIGHT);

    // Build router
    let app = Router::new()
        .route("/health", get(health))
        .route("/ready", get(health))
        .route("/api/weights", get(get_weights))
        .route("/api/score", post(calculate_score));

    // Start server
    let addr: SocketAddr = format!("0.0.0.0:{}", args.port).parse()?;
    info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
