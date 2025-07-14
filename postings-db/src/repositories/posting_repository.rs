use async_trait::async_trait;
use crate::models::posting::Posting;
use crate::DbError;
use uuid::Uuid;

#[async_trait]
pub trait PostingRepository {
    async fn find_by_opr_id_and_discarding_id_is_null(&self, opr_id: &str) -> Result<Option<Posting>, DbError>;
    async fn find_by_opr_id(&self, opr_id: &str) -> Result<Vec<Posting>, DbError>;
    async fn find_first_by_ledger_order_by_record_time_desc(&self, ledger_id: Uuid) -> Result<Option<Posting>, DbError>;
    async fn save(&self, posting: Posting) -> Result<Posting, DbError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Posting>, DbError>;
}
