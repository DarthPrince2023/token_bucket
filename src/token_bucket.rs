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

        // Calculate how much room is left in the bucket based on how many tokens have been used
        let capacity = self.max_tokens - self.current_counter;

        // Keep track of how many tokens created
        let mut generated_token_counter = 0;
        
        // Never add more tokens than allowed
        if tokens_created > capacity {
            // Add the allowed tokens and update refill time
            self.current_counter += capacity;
            generated_token_counter = capacity;
        } else {
            // Push the created tokens and update refill time
            self.current_counter += tokens_created;
            generated_token_counter = tokens_created;
        }
        
        // Check that we have enough tokens left
        if self.current_counter == 0 {
            // We do not have enough tokens to process this request
            return StatusCode::SERVICE_UNAVAILABLE;
        }

        if generated_token_counter > 0 {
            self.last_fill_time = Utc::now();
        }
        
        // Drop a token from the bucket
        self.current_counter -= 1;

        // Everything went well
        StatusCode::OK
    }
}