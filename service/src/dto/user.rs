use repository::entity::user::UserEntity;

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: i32,
    pub name: String,
}

impl From<UserEntity> for User {
    fn from(entity: UserEntity) -> Self {
        Self {
            id: entity.id,
            name: entity.name,
        }
    }
}

impl From<User> for UserEntity {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
        }
    }
}
