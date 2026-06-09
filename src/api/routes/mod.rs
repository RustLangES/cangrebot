mod daily_challenge;
use axum::http::StatusCode;
use axum::response::IntoResponse;
pub use daily_challenge::daily_challenge;

mod send_message;
pub use send_message::send_message;

mod showcase;
pub use showcase::showcase;

mod showcase_sync;
pub use showcase_sync::showcase_sync;

pub mod send_stats;

pub async fn healthcheck() -> impl IntoResponse {
    (StatusCode::OK, "Ok")
}
