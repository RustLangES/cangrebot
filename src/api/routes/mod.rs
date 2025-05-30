mod daily_challenge;
use axum::http::StatusCode;
use axum::response::IntoResponse;
pub use daily_challenge::daily_challenge;
mod send_message;
pub use send_message::send_message;

pub mod send_stats;

pub async fn healthcheck() -> impl IntoResponse {
    (StatusCode::OK, "Ok")
}
