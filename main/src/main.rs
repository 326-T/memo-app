//! Run with
//!
//! ```not_rust
//! cargo run -p example-hello-world
//! ```

use controller::routes::user::create_router;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = create_router();

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
