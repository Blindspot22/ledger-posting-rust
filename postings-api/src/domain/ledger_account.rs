use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::balance_side::BalanceSide;
use crate::domain::account_category::AccountCategory;
use crate::domain::chart_of_account::ChartOfAccount;
use crate::domain::ledger::Ledger;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LedgerAccount {
    pub id: Uuid,
    pub ledger: Ledger,
    pub parent: Option<Box<LedgerAccount>>,
    pub coa: ChartOfAccount,
    pub balance_side: BalanceSide,
    pub category: AccountCategory,
}
