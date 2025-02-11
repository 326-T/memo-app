use sqlx::PgPool;
use std::sync::Arc;
use testcontainers::{runners::AsyncRunner, ContainerAsync};
use testcontainers_modules::postgres::Postgres;

pub struct PostgresContainer {
    _container: ContainerAsync<Postgres>,
    pool: Arc<PgPool>,
}

impl PostgresContainer {
    pub async fn new() -> Self {
        let _container = Postgres::default()
            .with_user("postgres")
            .with_password("postgres")
            .with_db_name("postgres")
            .start()
            .await
            .unwrap();
        let connection_string = format!(
            "postgres://postgres:postgres@localhost:{}/postgres",
            _container.get_host_port_ipv4(5432).await.unwrap()
        );
        let pool = Arc::new(
            PgPool::connect(&connection_string)
                .await
                .expect("Failed to connect to Postgres"),
        );
        sqlx::migrate!("../migrations/schema")
            .run(pool.as_ref())
            .await
            .expect("Failed to run migrations");
        sqlx::migrate!("../migrations/test")
            .run(pool.as_ref())
            .await
            .expect("Failed to run migrations");

        Self { _container, pool }
    }

    pub fn pool(&self) -> Arc<PgPool> {
        self.pool.clone()
    }
}
