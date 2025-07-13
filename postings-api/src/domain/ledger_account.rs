use serde::{Deserialize, Serialize};
use crate::domain::balance_side::BalanceSide;
use crate::domain::account_category::AccountCategory;
use crate::domain::chart_of_account::ChartOfAccount;
use crate::domain::ledger::Ledger;
use crate::domain::named::Named;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LedgerAccount {
    #[serde(flatten)]
    pub named: Named,
    pub ledger: Ledger,
    pub parent: Option<Box<LedgerAccount>>,
    pub coa: ChartOfAccount,
    pub balance_side: BalanceSide,
    pub category: AccountCategory,
}
