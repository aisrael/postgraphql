use sqlx::mysql::MySqlRow;
use sqlx::postgres::PgRow;
use sqlx::{Column, Error, Row, TypeInfo};

pub fn pg_row_id(row: &PgRow) -> Result<i64, Error> {
    let id_column = row.column("id");
    let type_name = id_column.type_info().name();
    if type_name == "BIGINT UNSIGNED" {
        row.try_get::<i64, _>("id")
    } else if type_name == "INT4" {
        let i: i32 = row.try_get("id")?;
        Ok(i as i64)
    } else {
        Err(sqlx::Error::TypeNotFound {
            type_name: type_name.to_string(),
        })
    }
}

pub fn mysql_row_id(row: &MySqlRow) -> Result<i64, Error> {
    let id_column = row.column("id");
    let type_name = id_column.type_info().name();
    if type_name == "BIGINT UNSIGNED" {
        let u = row.try_get::<u64, _>("id")?;
        Ok(u as i64)
    } else if type_name == "INT" {
        row.try_get("id")
    } else {
        Err(Error::TypeNotFound {
            type_name: type_name.to_string(),
        })
    }
}
