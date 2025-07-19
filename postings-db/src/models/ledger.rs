use uuid::Uuid;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, PartialEq)]
pub struct Ledger {
    pub id: Uuid,
    pub coa_id: Uuid,
}
