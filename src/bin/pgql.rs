use axum::{Router, routing::get};
use postgraphql::connect;
use postgraphql::handlers::{authors, healthz};
use tokio::time::{Duration, timeout};

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./etc/migrations");
}

async fn initialize_db() {
    match connect().await {
        Ok(mut client) => {
            println!("Connected successfully!");
            let runner = embedded::migrations::runner();
            let duration = Duration::from_secs(60);
            if runner.get_migrations().is_empty() {
                println!("No migrations found");
            } else {
                println!("Found migrations:");
                for migration in runner.get_migrations() {
                    println!("{}", migration);
                }
            }
            match timeout(duration, runner.run_async(&mut client)).await {
                Ok(result) => match result {
                    Ok(report) => {
                        if report.applied_migrations().is_empty() {
                            println!("No migrations applied");
                        } else {
                            println!("Successfully applied migrations:");
                            for applied_migration in report.applied_migrations() {
                                println!("{}", applied_migration);
                            }
                        }
                    }
                    Err(e) => panic!("{}", e),
                },
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
    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/authors", get(authors));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
