use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, PartialEq)]
pub struct Ledger {
    pub id: String,
    pub coa_id: String,
}