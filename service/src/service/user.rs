use crate::dto::user::User;

#[mockall::automock]
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

    fn delete_user(&self, id: i32) {
        println!("Delete user with id: {}", id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_users() {
        // given
        let user_service = UserServiceImpl::default();
        // when
        let users = user_service.get_users();
        // then
        assert_eq!(users.len(), 2);
    }

    #[test]
    fn test_find_by_id() {
        // given
        let user_service = UserServiceImpl::default();
        let id = 1;
        // when
        let user = user_service.find_by_id(id);
        // then
        assert_eq!(user.id, id);
    }

    #[test]
    fn test_create_user() {
        // given
        let user_service = UserServiceImpl::default();
        let user = User {
            id: 0,
            name: "Charlie".to_string(),
        };
        // when
        let user = user_service.create_user(user);
        // then
        assert_eq!(user.id, 3);
        assert_eq!(user.name, "Charlie");
    }

    #[test]
    fn test_update_user() {
        // given
        let user_service = UserServiceImpl::default();
        let user = User {
            id: 1,
            name: "Alice".to_string(),
        };
        // when
        let user = user_service.update_user(user);
        // then
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "Alice");
    }

    #[test]
    fn test_delete_user() {
        // given
        let user_service = UserServiceImpl::default();
        let id = 1;
        // when
        user_service.delete_user(id);
        // then
        // no panic
    }
}
