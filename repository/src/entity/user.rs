#[derive(Debug, sqlx::FromRow)]
pub struct UserEntity {
    pub id: i32,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
