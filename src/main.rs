use std::net::SocketAddr;

pub mod state;
pub mod token_bucket;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Router};
pub use token_bucket::TokenBucket;

use crate::state::SharedState;

#[allow(unused)]
macro_rules! timeout {
    ($time:literal, $function:block) => {
        let duration = Duration::from_secs($time);
        let start = Instant::now();
        let stop = AtomicBool::new(false);
        let runner = start.elapsed();

        if let Some(time) = duration.checked_sub(runner) {
            thread::sleep(time);
        };

        thread::spawn(|| $function);

        if stop.load(Ordering::Relaxed) {
            return;
        }
        
        let _ = Box::new(move || {
            stop.store(true, Ordering::Relaxed)
        });
    };
}

#[tokio::main]
async fn main() {
    let state = SharedState::new();
    let router = Router::new()
        .route("/test", post(placeholder))
        .with_state(state);
    let addr = SocketAddr::from(([0, 0, 0, 0], 3030));
    let _ = axum_server::bind(addr).serve(router.into_make_service()).await;
}

async fn placeholder(State(shared_state): State<SharedState>) -> impl IntoResponse {
    shared_state.token_bucket.lock().await.request();
    StatusCode::OK
}
