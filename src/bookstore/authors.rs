use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlRow;
use sqlx::postgres::PgRow;
use sqlx::{Column, FromRow, Row, TypeInfo};

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    pub id: i64,
    pub name: String,
}

impl FromRow<'_, PgRow> for Author {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let id_column = row.column("id");
        let type_name = id_column.type_info().name();
        let id: i64 = if type_name == "BIGINT UNSIGNED" {
            row.try_get::<i64, _>("id")?
        } else if type_name == "INT4" {
            let i: i32 = row.try_get("id")?;
            i as i64
        } else {
            return Err(sqlx::Error::TypeNotFound {
                type_name: type_name.to_string(),
            });
        };
        let name = row.try_get::<String, _>("name")?;
        Ok(Author { id, name })
    }
}

impl FromRow<'_, MySqlRow> for Author {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        let id_column = row.column("id");
        let type_name = id_column.type_info().name();
        let id: i64 = if type_name == "BIGINT UNSIGNED" {
            let u = row.try_get::<u64, _>("id")?;
            u as i64
        } else if type_name == "I64" {
            row.try_get("id")?
        } else {
            return Err(sqlx::Error::TypeNotFound {
                type_name: type_name.to_string(),
            });
        };
        let name = row.try_get::<String, _>("name")?;
        Ok(Author { id, name })
    }
}
