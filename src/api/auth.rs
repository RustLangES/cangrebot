use axum::extract::{Request, State};
use axum::http::{HeaderMap, StatusCode};
use axum::middleware::Next;
use axum::response::Response;

use crate::CangrebotSecrets;

pub async fn middleware(
    State(secrets): State<CangrebotSecrets>,
    headers: HeaderMap,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let header_key = headers.get("Authorization");

    if header_key
        .as_ref()
        .is_some_and(|k| k.to_str().unwrap() == secrets.api_key)
    {
        return Ok(next.run(req).await);
    }

    tracing::error!("UNAUTHORIZED: {header_key:?}");

    Err(axum::http::StatusCode::UNAUTHORIZED)
}
