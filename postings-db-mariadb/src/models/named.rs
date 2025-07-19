use chrono::{DateTime, Utc};
use sqlx::FromRow;
use sqlx::Type;

#[derive(Debug, Clone, FromRow, PartialEq)]
pub struct Named {
    pub id: String,
    pub container: String,
    pub context: String,
    pub name: String,
    pub language: String,
    pub created: DateTime<Utc>,
    pub user_details: Vec<u8>,
    pub short_desc: Option<String>,
    pub long_desc: Option<String>,
    pub container_type: ContainerType,
}

#[derive(Debug, Clone, Type, PartialEq, Eq)]
#[sqlx(type_name = "container_type")]
pub enum ContainerType {
    ChartOfAccount,
    Ledger,
    LedgerAccount,
}