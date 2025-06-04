use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlRow;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};

use crate::db_utils::{mysql_row_id, pg_row_id};

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    pub id: i64,
    pub name: String,
}

impl FromRow<'_, PgRow> for Author {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let id = pg_row_id(row)?;
        let name = row.try_get::<String, _>("name")?;
        Ok(Author { id, name })
    }
}

impl FromRow<'_, MySqlRow> for Author {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        let id = mysql_row_id(row)?;
        let name = row.try_get::<String, _>("name")?;
        Ok(Author { id, name })
    }
}
