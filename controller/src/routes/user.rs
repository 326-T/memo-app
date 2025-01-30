use std::vec;

use axum::{extract::Path, http::StatusCode, routing::get, Json, Router};

use model::dto::user::{UserRequest, UserResponse};

pub fn create_router() -> Router {
    Router::new()
        .route("/users", get(get_users).post(create_user))
        .route(
            "/users/{id}",
            get(find_by_id).put(update_user).delete(delete_user),
        )
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

async fn create_user(Json(payload): Json<UserRequest>) -> (StatusCode, Json<UserResponse>) {
    (
        StatusCode::CREATED,
        Json(UserResponse {
            id: 1,
            name: payload.name.clone(),
        }),
    )
}

async fn update_user(Path(id): Path<i32>, Json(payload): Json<UserRequest>) -> Json<UserResponse> {
    Json(UserResponse {
        id,
        name: payload.name.clone(),
    })
}

async fn delete_user(Path(id): Path<i32>) -> StatusCode {
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
            .oneshot(
                Request::builder()
                    .uri("/users")
                    .body(Body::empty())
                    .unwrap(),
            )
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
            .oneshot(
                Request::builder()
                    .uri("/users/1")
                    .body(Body::empty())
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
    async fn test_create_user() {
        // given
        let app = super::create_router();
        // when
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/users")
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
                    .uri("/users/1")
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
                    .uri("/users/1")
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
