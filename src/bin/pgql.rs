use axum::{Router, routing::get};

use postgraphql::healthz;

#[tokio::main]
async fn main() {
    // Create our application with a single route
    let app = Router::new().route("/healthz", get(healthz));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
