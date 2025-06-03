use tokio_postgres::Client;
use tokio_postgres::tls::NoTls;

mod auth;
mod bookstore;

pub mod handlers;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("postgres error")]
    PostgresError(#[from] tokio_postgres::Error),
    #[error("sqlx error")]
    SqlxError(#[from] sqlx::Error),
}

pub async fn connect() -> Result<Client, Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=postgres dbname=postgraphql",
        NoTls,
    )
    .await
    .map_err(Error::PostgresError)?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
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
                    println!("{}", row.get(i).unwrap());
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
        let client = connect().await?;
        let rows = client.simple_query("SELECT 1").await?;
        for row in rows {
            debug(row)
        }
        Ok(())
    }
}
