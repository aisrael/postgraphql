use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};
use sqlx::mysql::MySqlRow;
use sqlx::postgres::PgRow;
use crate::db_utils::{mysql_row_id, pg_row_id};

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    pub id: i64,
    pub title: String,
    pub publish_year: u16,
    pub publish_month: u16,
}


impl FromRow<'_, PgRow> for Book {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let id = pg_row_id(row)?;
        let title = row.try_get::<String, _>("title")?;
        let publish_year = row.try_get::<i16, _>("publish_year")? as u16;
        let publish_month = row.try_get::<i16, _>("publish_month")? as u16;
        Ok(Book {
            id,
            title,
            publish_year,
            publish_month,
        })
    }
}

impl FromRow<'_, MySqlRow> for Book {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        let id = mysql_row_id(row)?;
        let title = row.try_get::<String, _>("title")?;
        let publish_year = row.try_get::<i32, _>("publish_year")? as u16;
        let publish_month = row.try_get::<i32, _>("publish_month")? as u16;
        Ok(Book {
            id,
            title,
            publish_year,
            publish_month,
        })
    }
}
