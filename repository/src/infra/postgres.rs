use sqlx::PgPool;
use std::sync::Arc;

pub async fn pool() -> Arc<PgPool> {
    Arc::new(
        PgPool::connect("postgres://postgres:postgres@localhost:5432/postgres")
            .await
            .expect("Failed to connect to Postgres"),
    )
}
