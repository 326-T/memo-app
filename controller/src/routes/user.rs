use crate::dto::user::{UserRequest, UserResponse};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use service::service::user::{UserService, UserServiceImpl};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    user_service: Arc<dyn UserService>,
}

pub fn create_router() -> Router {
    let user_service = Arc::new(UserServiceImpl::default());

    Router::new()
        .route("/", get(get_users).post(create_user))
        .route(
            "/{id}",
            get(find_by_id).put(update_user).delete(delete_user),
        )
        .with_state(AppState { user_service })
}

async fn get_users(State(AppState { user_service }): State<AppState>) -> Json<Vec<UserResponse>> {
    let users = user_service.get_users();
    let body = users.into_iter().map(|user| user.into()).collect();
    Json(body)
}

async fn find_by_id(
    State(AppState { user_service }): State<AppState>,
    Path(id): Path<i32>,
) -> Json<UserResponse> {
    let user = user_service.find_by_id(id);
    Json(UserResponse {
        id: user.id,
        name: user.name,
    })
}

async fn create_user(
    State(AppState { user_service }): State<AppState>,
    Json(payload): Json<UserRequest>,
) -> (StatusCode, Json<UserResponse>) {
    let user = user_service.create_user(payload.into());

    (StatusCode::CREATED, Json(user.into()))
}

async fn update_user(
    State(AppState { user_service }): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UserRequest>,
) -> Json<UserResponse> {
    let user = user_service.update_user(payload.into());
    Json(UserResponse {
        id: user.id,
        name: user.name,
    })
}

async fn delete_user(
    State(AppState { user_service }): State<AppState>,
    Path(id): Path<i32>,
) -> StatusCode {
    user_service.delete_user(id);
    StatusCode::NO_CONTENT
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use serde_json::{json, Value};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_get_users() {
        // given
        let app = super::create_router();
        // when
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        // then
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!([{"id":1,"name":"Alice"},{"id":2,"name":"Bob"}]));
    }

    #[tokio::test]
    async fn test_find_by_id() {
        // given
        let app = super::create_router();
        // when
        let response = app
            .oneshot(Request::builder().uri("/1").body(Body::empty()).unwrap())
            .await
            .unwrap();
        // then
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(body, r#"{"id":1,"name":"Alice"}"#);
    }

    #[tokio::test]
    async fn test_create_user() {
        // given
        let app = super::create_router();
        // when
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/")
                    .method(http::Method::POST)
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(json!({"name": "Alice"}).to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        // then
        assert_eq!(response.status(), StatusCode::CREATED);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(body, r#"{"id":1,"name":"Alice"}"#);
    }

    #[tokio::test]
    async fn test_update_user() {
        // given
        let app = super::create_router();
        // when
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/1")
                    .method(http::Method::PUT)
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(json!({"name": "Alice"}).to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        // then
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(body, r#"{"id":1,"name":"Alice"}"#);
    }

    #[tokio::test]
    async fn test_delete_user() {
        // given
        let app = super::create_router();
        // when
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/1")
                    .method(http::Method::DELETE)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        // then
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }
}
