use uuid::Uuid;
use sqlx::FromRow;
use crate::models::balance_side::BalanceSide;
use crate::models::account_category::AccountCategory;

#[derive(Debug, Clone, FromRow, PartialEq)]
pub struct LedgerAccount {
    pub id: Uuid,
    pub ledger_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub coa_id: Uuid,
    pub balance_side: BalanceSide,
    pub category: AccountCategory,
}
