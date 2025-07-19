pub mod domain;
pub mod service;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Database error")]
    Db,
    #[error("Not enough information provided")]
    NotEnoughInfo,
    #[error("Chart of account not found")]
    ChartOfAccountNotFound,
    #[error("Chart of account missmatch")]
    ChartOfAccountMismatch,
    #[error("Ledger account not found")]
    LedgerAccountNotFound,
    #[error("Ledger not found")]
    LedgerNotFound,
    #[error("Posting not found")]
    PostingNotFound,
    #[error("Double entry error: debits do not equal credits")]
    DoubleEntry,
    #[error("Posting time is before last closing")]
    BaselineTime,
    #[error("Posting time is missing")]
    PostingTimeMissing,
    #[error("No category defined for account")]
    NoCategory,
    #[error("Statement not found")]
    StatementNotFound,
    #[error("Statement is already closed")]
    StatementAlreadyClosed,
}
