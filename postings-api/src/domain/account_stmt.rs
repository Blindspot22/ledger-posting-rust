use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use crate::domain::financial_stmt::FinancialStmt;
use crate::domain::ledger_account::LedgerAccount;
use crate::domain::posting_trace::PostingTrace;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountStmt {
    #[serde(flatten)]
    pub financial_stmt: FinancialStmt,
    pub account: LedgerAccount,
    pub youngest_pst: Option<PostingTrace>,
    pub total_debit: BigDecimal,
    pub total_credit: BigDecimal,
}

impl AccountStmt {
    pub fn debit_balance(&self) -> BigDecimal {
        self.total_debit.clone() - self.total_credit.clone()
    }

    pub fn credit_balance(&self) -> BigDecimal {
        self.total_credit.clone() - self.total_debit.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::account_category::AccountCategory;
    use crate::domain::balance_side::BalanceSide;
    use crate::domain::chart_of_account::ChartOfAccount;
    use crate::domain::ledger::Ledger;
    use crate::domain::named::Named;
    use crate::domain::stmt_status::StmtStatus;
    use bigdecimal::BigDecimal;
    use chrono::Utc;
    use std::str::FromStr;
    use uuid::Uuid;

    fn create_test_account_stmt(total_debit: &str, total_credit: &str) -> AccountStmt {
        let now = Utc::now();
        let named = Named {
            id: Uuid::new_v4(),
            name: "Test".to_string(),
            created: now,
            user_details: "test_user".to_string(),
            short_desc: None,
            long_desc: None,
        };

        let coa = ChartOfAccount {
            named: named.clone(),
        };

        let ledger = Ledger {
            named: named.clone(),
            coa: coa.clone(),
        };

        let ledger_account = LedgerAccount {
            named: named.clone(),
            ledger: ledger.clone(),
            parent: None,
            coa: coa.clone(),
            balance_side: BalanceSide::Dr,
            category: AccountCategory::AS,
        };

        let financial_stmt = FinancialStmt {
            id: Uuid::new_v4(),
            posting: None,
            pst_time: now,
            stmt_status: StmtStatus::SIMULATED,
            latest_pst: None,
            stmt_seq_nbr: 1,
        };

        AccountStmt {
            financial_stmt,
            account: ledger_account,
            youngest_pst: None,
            total_debit: BigDecimal::from_str(total_debit).unwrap(),
            total_credit: BigDecimal::from_str(total_credit).unwrap(),
        }
    }

    #[test]
    fn test_debit_balance() {
        let stmt = create_test_account_stmt("100.00", "50.00");
        assert_eq!(
            stmt.debit_balance(),
            BigDecimal::from_str("50.00").unwrap()
        );
    }

    #[test]
    fn test_credit_balance() {
        let stmt = create_test_account_stmt("50.00", "100.00");
        assert_eq!(
            stmt.credit_balance(),
            BigDecimal::from_str("50.00").unwrap()
        );
    }

    #[test]
    fn test_zero_balance() {
        let stmt = create_test_account_stmt("100.00", "100.00");
        assert_eq!(stmt.debit_balance(), BigDecimal::from_str("0.00").unwrap());
        assert_eq!(
            stmt.credit_balance(),
            BigDecimal::from_str("0.00").unwrap()
        );
    }
}