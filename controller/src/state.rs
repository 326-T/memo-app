use repository::infra::postgres::pool;
use repository::repository::user::UserRepositoryImpl;
use service::service::user::{UserService, UserServiceImpl};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<dyn UserService>,
}

pub async fn state() -> AppState {
    let pool = pool().await;
    let user_repository = Arc::new(UserRepositoryImpl::new(pool));
    let user_service = Arc::new(UserServiceImpl::new(user_repository));
    AppState { user_service }
}
