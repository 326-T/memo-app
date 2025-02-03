use service::service::user::{UserService, UserServiceImpl};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<dyn UserService>,
}

pub fn state() -> AppState {
    let user_service = Arc::new(UserServiceImpl::default());
    AppState { user_service }
}
