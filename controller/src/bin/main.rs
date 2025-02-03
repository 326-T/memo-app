//! Run with
//!
//! ```not_rust
//! cargo run -p example-hello-world
//! ```

use axum::Router;
use controller::routes::user::sub_router;
use controller::state::state;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .nest("/users", sub_router())
        .with_state(state().await);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
