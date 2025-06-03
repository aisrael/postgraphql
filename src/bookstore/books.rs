use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    pub id: u32,
    pub name: String,
    pub publish_year: u16,
    pub publish_month: u16,
}
