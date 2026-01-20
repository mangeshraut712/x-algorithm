//! Thunder configuration

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ThunderConfig {
    pub max_posts: usize,
    pub retention_seconds: u64,
}
