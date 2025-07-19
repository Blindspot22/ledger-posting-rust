use async_trait::async_trait;
use crate::models::posting_line::PostingLine;
use crate::DbError;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[async_trait]
pub trait PostingLineRepository {
    async fn save(&self, posting_line: PostingLine) -> Result<PostingLine, DbError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<PostingLine>, DbError>;
    async fn find_by_account_and_pst_time_between(&self, account_id: Uuid, from: DateTime<Utc>, to: DateTime<Utc>) -> Result<Vec<PostingLine>, DbError>;
    async fn find_by_id_and_account_id(&self, id: Uuid, account_id: Uuid) -> Result<Option<PostingLine>, DbError>;
    async fn find_by_base_line_and_pst_time_less_than_equal(&self, base_line: Uuid, ref_time: DateTime<Utc>) -> Result<Vec<PostingLine>, DbError>;
    async fn find_by_account_and_pst_time_less_than_equal(&self, account_id: Uuid, ref_time: DateTime<Utc>) -> Result<Vec<PostingLine>, DbError>;
}
