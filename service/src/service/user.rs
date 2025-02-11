use crate::dto::user::User;
use repository::repository::user::UserRepository;
use shared::AppError;
use std::sync::Arc;

#[mockall::automock]
#[async_trait::async_trait]
pub trait UserService: Send + Sync {
    async fn get_users(&self) -> Result<Vec<User>, AppError>;
    async fn find_by_id(&self, id: i32) -> Result<Option<User>, AppError>;
    async fn create_user(&self, user: User) -> Result<User, AppError>;
    async fn update_user(&self, user: User) -> Result<User, AppError>;
    async fn delete_user(&self, id: i32) -> Result<(), AppError>;
}

#[derive(Clone)]
pub struct UserServiceImpl {
    user_repository: Arc<dyn UserRepository>,
}

impl UserServiceImpl {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
}

#[async_trait::async_trait]
impl UserService for UserServiceImpl {
    async fn get_users(&self) -> Result<Vec<User>, AppError> {
        self.user_repository
            .get_users()
            .await
            .map(|entities| entities.into_iter().map(User::from).collect())
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<User>, AppError> {
        self.user_repository
            .find_by_id(id)
            .await
            .map(|entity| entity.map(User::from))
    }

    async fn create_user(&self, user: User) -> Result<User, AppError> {
        self.user_repository
            .create_user(User::into(user))
            .await
            .map(User::from)
    }

    async fn update_user(&self, user: User) -> Result<User, AppError> {
        self.user_repository
            .update_user(User::into(user))
            .await
            .map(User::from)
    }

    async fn delete_user(&self, id: i32) -> Result<(), AppError> {
        self.user_repository.delete_user(id).await
    }
}

#[cfg(test)]
mod tests {
    use repository::{entity::user::UserEntity, repository::user::MockUserRepository};

    use super::*;

    #[tokio::test]
    async fn test_get_users() {
        // given
        let mut mock_user_repository = MockUserRepository::new();
        mock_user_repository.expect_get_users().returning(|| {
            Ok(vec![
                UserEntity {
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
                UserEntity {
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
        let user_service = UserServiceImpl::new(Arc::new(mock_user_repository));
        // when
        let users = user_service.get_users().await.unwrap();
        // then
        assert_eq!(users.len(), 2);
    }

    #[tokio::test]
    async fn test_find_by_id() {
        // given
        let mut mock_user_repository = MockUserRepository::new();
        mock_user_repository.expect_find_by_id().returning(|id| {
            Ok(Option::Some(UserEntity {
                id: id,
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
        let user_service = UserServiceImpl::new(Arc::new(mock_user_repository));
        let id = 1;
        // when
        let user = user_service.find_by_id(id).await.unwrap().unwrap();
        // then
        assert_eq!(user.id, id);
    }

    #[tokio::test]
    async fn test_create_user() {
        // given
        let mut mock_user_repository = MockUserRepository::new();
        mock_user_repository.expect_create_user().returning(|user| {
            Ok(UserEntity {
                id: 3,
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
        let user_service = UserServiceImpl::new(Arc::new(mock_user_repository));
        let user = User {
            id: 3,
            name: "Charlie".to_string(),
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
        };
        // when
        let user = user_service.create_user(user).await.unwrap();
        // then
        assert_eq!(user.id, 3);
        assert_eq!(user.name, "Charlie");
    }

    #[tokio::test]
    async fn test_update_user() {
        // given
        let mut mock_user_repository = MockUserRepository::new();
        mock_user_repository.expect_update_user().returning(|user| {
            Ok(UserEntity {
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
        let user_service = UserServiceImpl::new(Arc::new(mock_user_repository));
        let user = User {
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
        };
        // when
        let user = user_service.update_user(user).await.unwrap();
        // then
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "Alice");
    }

    #[tokio::test]
    async fn test_delete_user() {
        // given
        let mut mock_user_repository = MockUserRepository::new();
        mock_user_repository
            .expect_delete_user()
            .returning(|_| Ok(()));
        let user_service = UserServiceImpl::new(Arc::new(mock_user_repository));
        // when
        let _ = user_service.delete_user(1).await;
        // then
        // no panic
    }
}
