use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;
use sqlx::Type;

#[derive(Debug, Clone, FromRow, PartialEq)]
pub struct Named {
    pub id: Uuid,
    pub container: Uuid,
    pub context: Uuid,
    pub name: String,
    pub language: String,
    pub created: DateTime<Utc>,
    pub user_details: [u8; 34],
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