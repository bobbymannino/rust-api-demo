use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

// Default authorization token - in production use environment variables
const DEFAULT_AUTH_TOKEN: &str = "Bearer secret";

pub async fn auth_middleware(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|header| header.to_str().ok());

    match auth_header {
        Some(token) if token == DEFAULT_AUTH_TOKEN => Ok(next.run(request).await),
        Some(_) => {
            // Token exists but doesn't match
            Err(StatusCode::UNAUTHORIZED)
        }
        None => {
            // No authorization header provided
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}
