use crate::dto::user::User;

pub trait UserService: Send + Sync {
    fn get_users(&self) -> Vec<User>;
    fn find_by_id(&self, id: i32) -> User;
    fn create_user(&self, user: User) -> User;
    fn update_user(&self, user: User) -> User;
    fn delete_user(&self, id: i32);
}

#[derive(Clone, Default)]
pub struct UserServiceImpl {}

impl UserServiceImpl {}

impl UserService for UserServiceImpl {
    fn get_users(&self) -> Vec<User> {
        vec![
            User {
                id: 1,
                name: "Alice".to_string(),
            },
            User {
                id: 2,
                name: "Bob".to_string(),
            },
        ]
    }

    fn find_by_id(&self, id: i32) -> User {
        User {
            id,
            name: "Alice".to_string(),
        }
    }

    fn create_user(&self, user: User) -> User {
        User {
            id: 3,
            name: user.name.clone(),
        }
    }

    fn update_user(&self, user: User) -> User {
        User {
            id: user.id,
            name: user.name.clone(),
        }
    }

    fn delete_user(&self, id: i32) {}
}
