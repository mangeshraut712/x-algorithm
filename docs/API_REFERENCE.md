# API Reference

This document provides the API reference for the X Algorithm Rust services.

## HomeMixer HTTP API

The HomeMixer service provides HTTP endpoints for post scoring and algorithm configuration.

### Base URL
```
http://localhost:8080
```

### Endpoints

#### Health Check

```http
GET /health
```

**Response:**
```json
{
  "status": "ok",
  "version": "1.0.0"
}
```

---

#### Get Algorithm Weights

Retrieve the current algorithm weights used for scoring.

```http
GET /api/weights
```

**Response:**
```json
{
  "reply": 27.0,
  "profile_click": 12.0,
  "bookmark": 4.0,
  "follow_author": 4.0,
  "quote": 2.0,
  "dm_share": 2.0,
  "like": 1.0,
  "repost": 1.0,
  "video_view": 0.3,
  "not_interested": -74.0,
  "block": -150.0,
  "mute": -50.0,
  "report": -369.0
}
```

**Weight Explanations:**

| Weight | Description | Impact |
|--------|-------------|--------|
| `reply` (27.0) | Reply probability | Highest positive - drives conversation |
| `profile_click` (12.0) | Profile click probability | High - shows interest in author |
| `bookmark` (4.0) | Bookmark probability | Strong save intent |
| `follow_author` (4.0) | New follow probability | Strong author quality signal |
| `quote` (2.0) | Quote tweet probability | Adds conversation |
| `dm_share` (2.0) | DM share probability | High intent sharing |
| `like` (1.0) | Like probability | Base engagement |
| `repost` (1.0) | Repost probability | Distribution signal |
| `video_view` (0.3) | Video quality view | For eligible videos |
| `not_interested` (-74.0) | "Not interested" clicks | Very negative |
| `block` (-150.0) | Author blocks | Severe penalty |
| `mute` (-50.0) | Author mutes | Moderate penalty |
| `report` (-369.0) | Reports | Account-level penalty |

---

#### Calculate Post Score

Calculate the ranking score for a post based on engagement probabilities.

```http
POST /api/score
Content-Type: application/json
```

**Request Body:**
```json
{
  "reply_prob": 0.05,
  "like_prob": 0.15,
  "repost_prob": 0.02,
  "profile_click_prob": 0.08,
  "bookmark_prob": 0.03,
  "video_view_prob": 0.0,
  "has_link": false
}
```

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `reply_prob` | float | Yes | Probability of reply (0.0 - 1.0) |
| `like_prob` | float | Yes | Probability of like (0.0 - 1.0) |
| `repost_prob` | float | Yes | Probability of repost (0.0 - 1.0) |
| `profile_click_prob` | float | Yes | Probability of profile click (0.0 - 1.0) |
| `bookmark_prob` | float | Yes | Probability of bookmark (0.0 - 1.0) |
| `video_view_prob` | float | No | Video quality view probability (default: 0.0) |
| `has_link` | bool | No | Whether post contains external link (default: false) |

**Response:**
```json
{
  "score": 3.47,
  "breakdown": {
    "reply_contribution": 1.35,
    "profile_click_contribution": 0.96,
    "bookmark_contribution": 0.12,
    "like_contribution": 0.15,
    "repost_contribution": 0.02,
    "video_contribution": 0.0,
    "link_penalty": null
  },
  "tier": "AVERAGE"
}
```

**Score Tiers:**

| Tier | Score Range | Description |
|------|-------------|-------------|
| `VIRAL_POTENTIAL` | 30+ | Likely to gain significant reach |
| `GOOD` | 15-30 | Above average performance |
| `AVERAGE` | 5-15 | Normal distribution |
| `LOW` | 0-5 | Limited visibility |

---

## Thunder HTTP API

The Thunder service provides in-network post retrieval for the home timeline.

### Base URL
```
http://localhost:8080
```

### Configuration

Command-line arguments for the Thunder service:

| Argument | Default | Description |
|----------|---------|-------------|
| `--post-retention-seconds` | 604800 | Post retention period (7 days) |
| `--request-timeout-ms` | 5000 | Request timeout |
| `--grpc-port` | 50051 | gRPC server port |
| `--http-port` | 8080 | HTTP server port |
| `--result-limit` | 100 | Maximum results per query |

---

## Code Examples

### Rust Client

```rust
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ScoreRequest {
    reply_prob: f64,
    like_prob: f64,
    repost_prob: f64,
    profile_click_prob: f64,
    bookmark_prob: f64,
    has_link: bool,
}

#[derive(Deserialize)]
struct ScoreResponse {
    score: f64,
    tier: String,
}

async fn calculate_score() -> Result<ScoreResponse, reqwest::Error> {
    let client = Client::new();
    let req = ScoreRequest {
        reply_prob: 0.05,
        like_prob: 0.15,
        repost_prob: 0.02,
        profile_click_prob: 0.08,
        bookmark_prob: 0.03,
        has_link: false,
    };

    client
        .post("http://localhost:8080/api/score")
        .json(&req)
        .send()
        .await?
        .json()
        .await
}
```

### Python Client

```python
import requests

def calculate_score(reply_prob, like_prob, repost_prob, profile_click_prob, bookmark_prob, has_link=False):
    response = requests.post(
        "http://localhost:8080/api/score",
        json={
            "reply_prob": reply_prob,
            "like_prob": like_prob,
            "repost_prob": repost_prob,
            "profile_click_prob": profile_click_prob,
            "bookmark_prob": bookmark_prob,
            "has_link": has_link
        }
    )
    return response.json()

# Example usage
result = calculate_score(0.05, 0.15, 0.02, 0.08, 0.03)
print(f"Score: {result['score']}, Tier: {result['tier']}")
```

### JavaScript/Fetch

```javascript
async function calculateScore(engagement) {
    const response = await fetch('http://localhost:8080/api/score', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
            reply_prob: engagement.replyProb,
            like_prob: engagement.likeProb,
            repost_prob: engagement.repostProb,
            profile_click_prob: engagement.profileClickProb,
            bookmark_prob: engagement.bookmarkProb,
            has_link: engagement.hasLink || false
        })
    });
    return response.json();
}
```

### cURL

```bash
# Get weights
curl http://localhost:8080/api/weights

# Calculate score
curl -X POST http://localhost:8080/api/score \
  -H "Content-Type: application/json" \
  -d '{
    "reply_prob": 0.05,
    "like_prob": 0.15,
    "repost_prob": 0.02,
    "profile_click_prob": 0.08,
    "bookmark_prob": 0.03,
    "has_link": false
  }'
```

---

## Error Handling

### HTTP Status Codes

| Code | Description |
|------|-------------|
| 200 | Success |
| 400 | Bad Request - Invalid JSON or parameters |
| 500 | Internal Server Error |

### Error Response Format

```json
{
  "error": "Invalid request body",
  "details": "expected float for field 'reply_prob'"
}
```

---

## Rate Limiting

The API currently does not implement rate limiting. For production use, consider implementing rate limiting at the reverse proxy level (nginx, envoy, etc.).

---

## Versioning

The API follows semantic versioning. The current version is `1.0.0`. Breaking changes will result in a major version bump.
