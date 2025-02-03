#[derive(Debug, sqlx::FromRow)]
pub struct UserEntity {
    pub id: i32,
    pub name: String,
}
