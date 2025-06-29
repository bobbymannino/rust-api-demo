use axum::{extract::Path, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

pub async fn get_user(Path(user_id): Path<u32>) -> Json<User> {
    println!("Get user {:?}", user_id);

    let user = User {
        id: user_id,
        name: "John Ward".to_string(),
        email: "johnward@gmail.com".to_string(),
    };

    Json(user)
}

pub async fn create_user(Json(payload): Json<User>) -> Json<User> {
    println!("Created user: {:?}", payload.name);

    Json(payload)
}

pub async fn delete_user(Path(user_id): Path<u32>) -> StatusCode {
    println!("Deleted user: {}", user_id);

    StatusCode::OK
}
