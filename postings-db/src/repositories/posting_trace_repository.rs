use async_trait::async_trait;
use crate::models::posting_trace::PostingTrace;
use crate::DbError;

#[async_trait]
pub trait PostingTraceRepository {
    async fn save(&self, trace: PostingTrace) -> Result<PostingTrace, DbError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<PostingTrace>, DbError>;
}
