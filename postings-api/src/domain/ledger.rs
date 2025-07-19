use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::chart_of_account::ChartOfAccount;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Ledger {
    pub id: Uuid,
    pub coa: ChartOfAccount,
}
