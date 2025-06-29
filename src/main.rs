mod auth;
mod user;

use crate::auth::auth_middleware;
use crate::user::{create_user, delete_user, get_user};
use axum::{
    Router, middleware,
    routing::{get, post},
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/users/{user_id}", get(get_user).delete(delete_user))
        .route("/users", post(create_user))
        .layer(middleware::from_fn(auth_middleware));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3005").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
