use serde::{Deserialize, Serialize};
use crate::domain::chart_of_account::ChartOfAccount;
use crate::domain::named::Named;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Ledger {
    #[serde(flatten)]
    pub named: Named,
    pub coa: ChartOfAccount,
}
