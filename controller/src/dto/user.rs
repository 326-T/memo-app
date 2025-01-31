use serde::{Deserialize, Serialize};
use service::dto::user::User;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserRequest {
    pub name: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
        }
    }
}

impl Into<User> for UserRequest {
    fn into(self) -> User {
        User {
            id: 0,
            name: self.name,
        }
    }
}
