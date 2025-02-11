use crate::entity::user::UserEntity;
use shared::AppError;
use sqlx::PgPool;
use std::sync::Arc;

#[mockall::automock]
#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_users(&self) -> Result<Vec<UserEntity>, AppError>;
    async fn find_by_id(&self, id: i32) -> Result<Option<UserEntity>, AppError>;
    async fn create_user(&self, user: UserEntity) -> Result<UserEntity, AppError>;
    async fn update_user(&self, user: UserEntity) -> Result<UserEntity, AppError>;
    async fn delete_user(&self, id: i32) -> Result<(), AppError>;
}

#[derive(Debug, Clone)]
pub struct UserRepositoryImpl {
    pub db: Arc<PgPool>,
}

impl UserRepositoryImpl {
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn get_users(&self) -> Result<Vec<UserEntity>, AppError> {
        let entities = sqlx::query_as::<_, UserEntity>("SELECT * FROM users;")
            .fetch_all(&*self.db)
            .await?;
        Ok(entities)
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<UserEntity>, AppError> {
        let entity = sqlx::query_as::<_, UserEntity>("SELECT * FROM users WHERE id = $1;")
            .bind(id)
            .fetch_optional(&*self.db)
            .await?;
        Ok(entity)
    }

    async fn create_user(&self, user: UserEntity) -> Result<UserEntity, AppError> {
        let entity =
            sqlx::query_as::<_, UserEntity>("INSERT INTO users (name) VALUES ($1) RETURNING *;")
                .bind(&user.name)
                .fetch_one(&*self.db)
                .await?;
        Ok(entity)
    }

    async fn update_user(&self, user: UserEntity) -> Result<UserEntity, AppError> {
        let entity = sqlx::query_as::<_, UserEntity>(
            "UPDATE users SET name = $1 WHERE id = $2 RETURNING *;",
        )
        .bind(&user.name)
        .bind(user.id)
        .fetch_one(&*self.db)
        .await?;
        Ok(entity)
    }

    async fn delete_user(&self, id: i32) -> Result<(), AppError> {
        sqlx::query("DELETE FROM users WHERE id = $1;")
            .bind(id)
            .execute(&*self.db)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::infra::testcontainer::PostgresContainer;

    #[tokio::test]
    async fn test_get_users() {
        // given
        let container = PostgresContainer::new().await;
        let repository = UserRepositoryImpl::new(container.pool());
        // when
        let users = repository.get_users().await.unwrap();
        // then
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].id, 1);
        assert_eq!(users[0].name, "Alice");
    }

    #[tokio::test]
    async fn test_find_by_id() {
        // given
        let container = PostgresContainer::new().await;
        let repository = UserRepositoryImpl::new(container.pool());
        // when
        let user = repository.find_by_id(1).await.unwrap();
        // then
        assert!(user.is_some());
        let user = user.unwrap();
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "Alice");
    }

    #[tokio::test]
    async fn test_create_user() {
        // given
        let container = PostgresContainer::new().await;
        let repository = UserRepositoryImpl::new(container.pool());
        // when
        let user = repository
            .create_user(UserEntity {
                id: 2,
                name: "Bob".to_string(),
                created_at: chrono::Utc::now().naive_utc(),
                updated_at: chrono::Utc::now().naive_utc(),
            })
            .await
            .unwrap();
        // then
        assert_eq!(user.id, 2);
        assert_eq!(user.name, "Bob");
    }

    #[tokio::test]
    async fn test_update_user() {
        // given
        let container = PostgresContainer::new().await;
        let repository = UserRepositoryImpl::new(container.pool());
        // when
        let user = repository
            .update_user(UserEntity {
                id: 1,
                name: "Charlie".to_string(),
                created_at: chrono::Utc::now().naive_utc(),
                updated_at: chrono::Utc::now().naive_utc(),
            })
            .await
            .unwrap();
        // then
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "Charlie");
    }

    #[tokio::test]
    async fn test_delete_user() {
        // given
        let container = PostgresContainer::new().await;
        let repository = UserRepositoryImpl::new(container.pool());
        // when
        let _ = repository.delete_user(1).await;
        // then
        // no assertion
    }
}
