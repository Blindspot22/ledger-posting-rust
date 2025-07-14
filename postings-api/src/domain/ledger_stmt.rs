use serde::{Deserialize, Serialize};
use crate::domain::financial_stmt::FinancialStmt;
use crate::domain::ledger::Ledger;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LedgerStmt {
    #[serde(flatten)]
    pub financial_stmt: FinancialStmt,
    pub ledger: Ledger,
}
