use std::sync::Arc;

use chrono::Utc;
use tokio::sync::Mutex;

use crate::TokenBucket;

#[derive(Debug, Clone)]
pub struct SharedState {
    pub token_bucket: Arc<Mutex<TokenBucket>>
}

impl SharedState {
    pub fn new() -> Self {
        let bucket = TokenBucket {
            max_tokens: 1_000,
            current_counter: 1_000,
            refill_rate: 11,
            last_fill_time: Utc::now()
        };

        Self {
            token_bucket: Arc::new(Mutex::new(bucket))
        }
    }
}