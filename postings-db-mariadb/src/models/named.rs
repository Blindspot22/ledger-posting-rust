use chrono::{DateTime, Utc};
use sqlx::FromRow;

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
    pub container_type: String,  // Will convert to/from ContainerType enum
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContainerType {
    ChartOfAccount,
    Ledger,
    LedgerAccount,
}

impl From<String> for ContainerType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "ChartOfAccount" => ContainerType::ChartOfAccount,
            "Ledger" => ContainerType::Ledger,
            "LedgerAccount" => ContainerType::LedgerAccount,
            _ => panic!("Unknown container type: {}", s),
        }
    }
}

impl From<ContainerType> for String {
    fn from(ct: ContainerType) -> Self {
        match ct {
            ContainerType::ChartOfAccount => "ChartOfAccount".to_string(),
            ContainerType::Ledger => "Ledger".to_string(),
            ContainerType::LedgerAccount => "LedgerAccount".to_string(),
        }
    }
}