use crate::dto::user::{UserRequest, UserResponse};
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use service::dto::user::User;

pub fn sub_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_users).post(create_user))
        .route(
            "/{id}",
            get(find_by_id).put(update_user).delete(delete_user),
        )
}

async fn get_users(State(AppState { user_service }): State<AppState>) -> Json<Vec<UserResponse>> {
    let users = user_service.get_users().await.unwrap();
    let body = users.into_iter().map(|user| user.into()).collect();
    Json(body)
}

async fn find_by_id(
    State(AppState { user_service }): State<AppState>,
    Path(id): Path<i32>,
) -> Json<UserResponse> {
    let user = user_service.find_by_id(id).await.unwrap();
    Json(user.unwrap().into())
}

async fn create_user(
    State(AppState { user_service }): State<AppState>,
    Json(payload): Json<UserRequest>,
) -> (StatusCode, Json<UserResponse>) {
    let user = user_service.create_user(payload.into()).await.unwrap();

    (StatusCode::CREATED, Json(user.into()))
}

async fn update_user(
    State(AppState { user_service }): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UserRequest>,
) -> Json<UserResponse> {
    let mut user: User = payload.into();
    user.id = id;
    let user = user_service.update_user(user).await.unwrap();
    Json(user.into())
}

async fn delete_user(
    State(AppState { user_service }): State<AppState>,
    Path(id): Path<i32>,
) -> StatusCode {
    user_service.delete_user(id).await.unwrap();
    StatusCode::NO_CONTENT
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use serde_json::{json, Value};
    use service::{dto::user::User, service::user::MockUserService};
    use std::sync::Arc;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_get_users() {
        // given
        let mut mock_user_service = MockUserService::new();
        mock_user_service.expect_get_users().returning(|| {
            Ok(vec![
                User {
                    id: 1,
                    name: "Alice".to_string(),
                    created_at: chrono::NaiveDateTime::parse_from_str(
                        "2021-01-01 00:00:00",
                        "%Y-%m-%d %H:%M:%S",
                    )
                    .unwrap(),
                    updated_at: chrono::NaiveDateTime::parse_from_str(
                        "2021-01-01 00:00:00",
                        "%Y-%m-%d %H:%M:%S",
                    )
                    .unwrap(),
                },
                User {
                    id: 2,
                    name: "Bob".to_string(),
                    created_at: chrono::NaiveDateTime::parse_from_str(
                        "2021-01-01 00:00:00",
                        "%Y-%m-%d %H:%M:%S",
                    )
                    .unwrap(),
                    updated_at: chrono::NaiveDateTime::parse_from_str(
                        "2021-01-01 00:00:00",
                        "%Y-%m-%d %H:%M:%S",
                    )
                    .unwrap(),
                },
            ])
        });
        let app = sub_router().with_state(AppState {
            user_service: Arc::new(mock_user_service),
        });
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
        let mut mock_user_service = MockUserService::new();
        mock_user_service.expect_find_by_id().returning(|id| {
            Ok(Some(User {
                id,
                name: "Alice".to_string(),
                created_at: chrono::NaiveDateTime::parse_from_str(
                    "2021-01-01 00:00:00",
                    "%Y-%m-%d %H:%M:%S",
                )
                .unwrap(),
                updated_at: chrono::NaiveDateTime::parse_from_str(
                    "2021-01-01 00:00:00",
                    "%Y-%m-%d %H:%M:%S",
                )
                .unwrap(),
            }))
        });
        let app = sub_router().with_state(AppState {
            user_service: Arc::new(mock_user_service),
        });
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
        let mut mock_user_service = MockUserService::new();
        mock_user_service.expect_create_user().returning(|user| {
            Ok(User {
                id: 2,
                name: user.name.clone(),
                created_at: chrono::NaiveDateTime::parse_from_str(
                    "2021-01-01 00:00:00",
                    "%Y-%m-%d %H:%M:%S",
                )
                .unwrap(),
                updated_at: chrono::NaiveDateTime::parse_from_str(
                    "2021-01-01 00:00:00",
                    "%Y-%m-%d %H:%M:%S",
                )
                .unwrap(),
            })
        });
        let app = sub_router().with_state(AppState {
            user_service: Arc::new(mock_user_service),
        });
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
        assert_eq!(body, r#"{"id":2,"name":"Alice"}"#);
    }

    #[tokio::test]
    async fn test_update_user() {
        // given
        let mut mock_user_service = MockUserService::new();
        mock_user_service.expect_update_user().returning(|user| {
            Ok(User {
                id: user.id,
                name: user.name.clone(),
                created_at: chrono::NaiveDateTime::parse_from_str(
                    "2021-01-01 00:00:00",
                    "%Y-%m-%d %H:%M:%S",
                )
                .unwrap(),
                updated_at: chrono::NaiveDateTime::parse_from_str(
                    "2021-01-01 00:00:00",
                    "%Y-%m-%d %H:%M:%S",
                )
                .unwrap(),
            })
        });
        let app = sub_router().with_state(AppState {
            user_service: Arc::new(mock_user_service),
        });
        // when
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/3")
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
        assert_eq!(body, r#"{"id":3,"name":"Alice"}"#);
    }

    #[tokio::test]
    async fn test_delete_user() {
        // given
        let mut mock_user_service = MockUserService::new();
        mock_user_service.expect_delete_user().returning(|_| Ok(()));
        let app = sub_router().with_state(AppState {
            user_service: Arc::new(mock_user_service),
        });
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
