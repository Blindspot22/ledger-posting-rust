use async_trait::async_trait;
use chrono::{DateTime, Utc};
use crate::domain::ledger_account::LedgerAccount;
use crate::domain::posting::Posting;
use crate::domain::posting_line::PostingLine;
use crate::ServiceError;
use uuid::Uuid;

pub struct Page<T> {
    pub content: Vec<T>,
    pub total_elements: u64,
}

#[async_trait]
pub trait PostingService {
    async fn new_posting(&self, posting: Posting) -> Result<Posting, ServiceError>;
    async fn find_postings_by_operation_id(&self, opr_id: &str) -> Result<Vec<Posting>, ServiceError>;
    async fn find_postings_by_dates(&self, ledger_account: LedgerAccount, date_from: DateTime<Utc>, date_to: DateTime<Utc>) -> Result<Vec<PostingLine>, ServiceError>;
    async fn find_postings_by_dates_paged(&self, ledger_account: LedgerAccount, date_from: DateTime<Utc>, date_to: DateTime<Utc>, page: usize, size: usize) -> Result<Page<PostingLine>, ServiceError>;
    async fn find_posting_line_by_id(&self, ledger_account: LedgerAccount, transaction_id: Uuid) -> Result<PostingLine, ServiceError>;
}
