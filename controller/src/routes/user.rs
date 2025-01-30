use std::vec;

use axum::{
    extract::Path,
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};

use model::dto::user::{UserRequest, UserResponse};

pub fn create_router() -> Router {
    Router::new()
        .route("/users", get(get_users))
        .route("/users/{id}", get(find_by_id))
        .route("/users", post(create_user))
        .route("/users/{id}", put(update_user))
        .route("/users/{id}", delete(delete_user))
}

async fn get_users() -> Json<Vec<UserResponse>> {
    Json(vec![
        UserResponse {
            id: 1,
            name: "Alice".to_string(),
        },
        UserResponse {
            id: 2,
            name: "Bob".to_string(),
        },
    ])
}

async fn find_by_id(Path(id): Path<i32>) -> Json<UserResponse> {
    Json(UserResponse {
        id,
        name: "Alice".to_string(),
    })
}

async fn create_user(Json(payload): Json<UserRequest>) -> Json<UserResponse> {
    Json(UserResponse {
        id: 3,
        name: payload.name,
    })
}

async fn update_user(Path(id): Path<i32>, Json(payload): Json<UserRequest>) -> Json<UserResponse> {
    Json(UserResponse {
        id,
        name: payload.name,
    })
}

async fn delete_user(Path(id): Path<i32>) -> StatusCode {
    StatusCode::NO_CONTENT
}
