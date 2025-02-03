use crate::dto::user::User;
use repository::repository::user::UserRepository;
use std::sync::Arc;

#[mockall::automock]
#[async_trait::async_trait]
pub trait UserService: Send + Sync {
    async fn get_users(&self) -> Vec<User>;
    async fn find_by_id(&self, id: i32) -> Option<User>;
    async fn create_user(&self, user: User) -> User;
    async fn update_user(&self, user: User) -> User;
    async fn delete_user(&self, id: i32);
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
    async fn get_users(&self) -> Vec<User> {
        self.user_repository
            .get_users()
            .await
            .into_iter()
            .map(User::from)
            .collect()
    }

    async fn find_by_id(&self, id: i32) -> Option<User> {
        let entity = self.user_repository.find_by_id(id).await;
        entity.map(User::from)
    }

    async fn create_user(&self, user: User) -> User {
        let entity = self.user_repository.create_user(User::into(user)).await;
        User::from(entity)
    }

    async fn update_user(&self, user: User) -> User {
        let entity = self.user_repository.update_user(User::into(user)).await;
        User::from(entity)
    }

    async fn delete_user(&self, id: i32) {
        self.user_repository.delete_user(id).await;
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
            vec![
                UserEntity {
                    id: 1,
                    name: "Alice".to_string(),
                },
                UserEntity {
                    id: 2,
                    name: "Bob".to_string(),
                },
            ]
        });
        let user_service = UserServiceImpl::new(Arc::new(mock_user_repository));
        // when
        let users = user_service.get_users().await;
        // then
        assert_eq!(users.len(), 2);
    }

    #[tokio::test]
    async fn test_find_by_id() {
        // given
        let mut mock_user_repository = MockUserRepository::new();
        mock_user_repository.expect_find_by_id().returning(|id| {
            Option::Some(UserEntity {
                id: id,
                name: "Alice".to_string(),
            })
        });
        let user_service = UserServiceImpl::new(Arc::new(mock_user_repository));
        let id = 1;
        // when
        let user = user_service.find_by_id(id).await.unwrap();
        // then
        assert_eq!(user.id, id);
    }

    #[tokio::test]
    async fn test_create_user() {
        // given
        let mut mock_user_repository = MockUserRepository::new();
        mock_user_repository
            .expect_create_user()
            .returning(|user| UserEntity {
                id: 3,
                name: user.name.clone(),
            });
        let user_service = UserServiceImpl::new(Arc::new(mock_user_repository));
        let user = User {
            id: 3,
            name: "Charlie".to_string(),
        };
        // when
        let user = user_service.create_user(user).await;
        // then
        assert_eq!(user.id, 3);
        assert_eq!(user.name, "Charlie");
    }

    #[tokio::test]
    async fn test_update_user() {
        // given
        let mut mock_user_repository = MockUserRepository::new();
        mock_user_repository
            .expect_update_user()
            .returning(|user| UserEntity {
                id: user.id,
                name: user.name.clone(),
            });
        let user_service = UserServiceImpl::new(Arc::new(mock_user_repository));
        let user = User {
            id: 1,
            name: "Alice".to_string(),
        };
        // when
        let user = user_service.update_user(user).await;
        // then
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "Alice");
    }

    #[tokio::test]
    async fn test_delete_user() {
        // given
        let mut mock_user_repository = MockUserRepository::new();
        mock_user_repository.expect_delete_user().returning(|_| {});
        let user_service = UserServiceImpl::new(Arc::new(mock_user_repository));
        // when
        user_service.delete_user(1).await;
        // then
        // no panic
    }
}
