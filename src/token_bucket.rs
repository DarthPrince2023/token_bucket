use axum::{http::StatusCode, response::IntoResponse};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct TokenBucket {
    pub max_tokens: i64,
    pub current_counter: i64,
    pub refill_rate: i64,
    pub last_fill_time: DateTime<Utc>
}

impl TokenBucket {
    pub fn new(max_tokens: i64, current_counter: i64, refill_rate: i64) -> Self {
        let last_fill_time = Utc::now();
        Self {
            max_tokens, current_counter, refill_rate, last_fill_time
        }
    }
    
    pub fn request(&mut self) -> impl IntoResponse {
        // Get the current time and subtract that from the previous refill time
        let time = Utc::now()
            .timestamp_millis() - self.last_fill_time.timestamp_millis();

        // Next calculate how many tokens were created since the last refill time
        let rate: i64 = self.refill_rate;
        let tokens_created = time / rate;
        
        // Check that we have enough tokens left
        if self.current_counter == 0 {
            // We do not have enough tokens to process this request
            return StatusCode::SERVICE_UNAVAILABLE;
        }
        
        // Drop a token from the bucket
        self.current_counter -= 1;

        // Calculate how much room is left in the bucket based on how many tokens have been used
        let capacity = self.max_tokens - self.current_counter;

        // Never add more tokens than allowed
        if tokens_created > capacity {
            self.current_counter += capacity;
        }

        // Update the last refill time
        self.last_fill_time = Utc::now();

        // Everything went well
        StatusCode::OK
    }
}