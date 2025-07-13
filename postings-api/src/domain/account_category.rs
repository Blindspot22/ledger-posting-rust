use serde::{Deserialize, Serialize};
use crate::domain::balance_side::BalanceSide;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccountCategory {
    RE, // Revenue
    EX, // Expense
    AS, // Asset
    LI, // Liability
    EQ, // Equity
    NOOP, // Non-Operating Income or Expenses
    NORE, // Non-Operating Revenue
    NOEX, // Non-Operating Expenses
}

impl fmt::Display for AccountCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl AccountCategory {
    pub fn desc(&self) -> &'static str {
        match self {
            AccountCategory::RE => "Revenue",
            AccountCategory::EX => "Expense",
            AccountCategory::AS => "Asset",
            AccountCategory::LI => "Liability",
            AccountCategory::EQ => "Equity",
            AccountCategory::NOOP => "Non-Operating Income or Expenses",
            AccountCategory::NORE => "Non-Operating Revenue",
            AccountCategory::NOEX => "Non-Operating Expenses",
        }
    }

    pub fn default_bs(&self) -> BalanceSide {
        match self {
            AccountCategory::RE => BalanceSide::Cr,
            AccountCategory::EX => BalanceSide::Dr,
            AccountCategory::AS => BalanceSide::Dr,
            AccountCategory::LI => BalanceSide::Cr,
            AccountCategory::EQ => BalanceSide::Cr,
            AccountCategory::NOOP => BalanceSide::DrCr,
            AccountCategory::NORE => BalanceSide::Cr,
            AccountCategory::NOEX => BalanceSide::Dr,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_bs() {
        assert_eq!(AccountCategory::RE.default_bs(), BalanceSide::Cr);
        assert_eq!(AccountCategory::EX.default_bs(), BalanceSide::Dr);
        assert_eq!(AccountCategory::AS.default_bs(), BalanceSide::Dr);
        assert_eq!(AccountCategory::LI.default_bs(), BalanceSide::Cr);
        assert_eq!(AccountCategory::EQ.default_bs(), BalanceSide::Cr);
        assert_eq!(AccountCategory::NOOP.default_bs(), BalanceSide::DrCr);
        assert_eq!(AccountCategory::NORE.default_bs(), BalanceSide::Cr);
        assert_eq!(AccountCategory::NOEX.default_bs(), BalanceSide::Dr);
    }
}