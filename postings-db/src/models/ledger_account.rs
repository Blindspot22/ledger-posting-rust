use sqlx::FromRow;
use crate::models::balance_side::BalanceSide;
use crate::models::account_category::AccountCategory;

#[derive(Debug, Clone, FromRow, PartialEq)]
pub struct LedgerAccount {
    pub id: String,
    pub name: String,
    pub ledger_id: String,
    pub parent_id: Option<String>,
    pub coa_id: String,
    pub balance_side: BalanceSide,
    pub category: AccountCategory,
    pub created: chrono::DateTime<chrono::Utc>,
    pub user_details: String,
    pub short_desc: Option<String>,
    pub long_desc: Option<String>,
}
