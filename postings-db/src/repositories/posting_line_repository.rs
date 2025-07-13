use async_trait::async_trait;
use crate::models::posting_line::PostingLine;
use crate::DbError;
use chrono::{DateTime, Utc};

#[async_trait]
pub trait PostingLineRepository {
    async fn find_by_account_and_pst_time_between(&self, account_id: &str, from: DateTime<Utc>, to: DateTime<Utc>) -> Result<Vec<PostingLine>, DbError>;
    async fn find_by_id_and_account_id(&self, id: &str, account_id: &str) -> Result<Option<PostingLine>, DbError>;
    async fn find_by_base_line_and_pst_time_less_than_equal(&self, base_line: &str, ref_time: DateTime<Utc>) -> Result<Vec<PostingLine>, DbError>;
    async fn find_by_account_and_pst_time_less_than_equal(&self, account_id: &str, ref_time: DateTime<Utc>) -> Result<Vec<PostingLine>, DbError>;
}
