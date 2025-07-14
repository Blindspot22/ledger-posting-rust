use uuid::Uuid;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, PartialEq)]
pub struct Ledger {
    pub id: Uuid,
    pub name: String,
    pub coa_id: Uuid,
    pub created: chrono::DateTime<chrono::Utc>,
    pub user_details: String,
    pub short_desc: Option<String>,
    pub long_desc: Option<String>,
}
