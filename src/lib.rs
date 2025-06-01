use tokio_postgres::tls::{NoTls, TlsStream};
use tokio_postgres::{Client, Connection, Socket};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("postgres error")]
    PostgresError(#[from] tokio_postgres::Error),
}

// Handler for the health check endpoint, just returns HTTP 200 "OK"
pub async fn healthz() -> &'static str {
    "OK"
}

pub async fn connect() -> Result<(Client, Connection<Socket, impl TlsStream>), Error> {
    tokio_postgres::connect(
        "host=localhost user=postgres password=postgres dbname=postgraphql",
        NoTls,
    )
    .await
    .map_err(|e| Error::PostgresError(e))
}

#[cfg(test)]
mod tests {
    use tokio_postgres::SimpleQueryMessage;
    use tokio_postgres::SimpleQueryMessage::{CommandComplete, Row, RowDescription};

    use super::*;

    fn debug(sqm: SimpleQueryMessage) {
        match sqm {
            Row(row) => {
                println!("columns: {:?}", row.columns());
                println!("len: {:?}", row.columns());
                for i in 0..row.columns().len() {
                    println!("column {}: {:?}", i, row.columns()[i]);
                    println!("{}", row.get(i).unwra qp());
                }
            }
            CommandComplete(count) => {
                println!("CommandComplete: {:?}", count);
            }
            RowDescription(arc) => {
                println!("RowDescription: {:?}", arc);
                let columns = arc.as_ref();
                for column in columns {
                    println!("Column: {:?}", column);
                }
            }
            _ => panic!("unexpected row kind"),
        }
    }

    #[tokio::test]
    async fn test_pg() -> Result<(), Error> {
        let (client, connection) = connect().await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        let rows = client.simple_query("SELECT 1").await?;
        for row in rows {
            debug(row)
        }
        Ok(())
    }
}
