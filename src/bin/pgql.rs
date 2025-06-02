use axum::{routing::get, Router};
use postgraphql::{connect, healthz};
use tokio::time::{timeout, Duration};

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./etc/migrations");
}

async fn initialize_db() {
    match connect().await {
        Ok(mut client) => {
            println!("Connected successfully!");
            let runner = embedded::migrations::runner();
            let duration = Duration::from_secs(10);
            match timeout(duration, runner.run_async(&mut client)).await {
                Ok(r) => {
                    println!("{:?}", r);
                    println!("Successfully applied migrations");
                }
                Err(e) => panic!("{}", e),
            }
        }
        Err(e) => {
            panic!("{}", e);
        }
    }
}

#[tokio::main]
async fn main() {
    initialize_db().await;

    // Create our application with a single route
    let app = Router::new().route("/healthz", get(healthz));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
