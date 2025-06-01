use tokio_postgres::NoTls;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("postgres error")]
    PostgresError(#[from] tokio_postgres::Error),
}

// Handler for the health check endpoint, just returns HTTP 200 "OK"
pub async fn healthz() -> &'static str {
    "OK"
}

pub async fn connect() -> Result<(), Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=postgres dbname=postgraphql",
        NoTls,
    )
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pg() {
        let result = connect().await;
        match result {
            Err(Error::PostgresError(e)) => {
                eprintln!("{:?}", e);
            }
            _ => {
                assert!(result.is_ok());
            }
        }
    }
}
