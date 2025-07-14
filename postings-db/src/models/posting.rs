use uuid::Uuid;
use sqlx::FromRow;
use crate::models::posting_type::PostingType;
use crate::models::posting_status::PostingStatus;

#[derive(Debug, Clone, FromRow, PartialEq)]
pub struct Posting {
    pub id: Uuid,
    pub record_user: String,
    pub record_time: chrono::DateTime<chrono::Utc>,
    pub opr_id: String,
    pub opr_time: chrono::DateTime<chrono::Utc>,
    pub opr_type: String,
    pub opr_details: Option<String>,
    pub opr_src: Option<String>,
    pub pst_time: chrono::DateTime<chrono::Utc>,
    pub pst_type: PostingType,
    pub pst_status: PostingStatus,
    pub ledger_id: Uuid,
    pub val_time: Option<chrono::DateTime<chrono::Utc>>,
    pub discarded_id: Option<Uuid>,
    pub discarded_time: Option<chrono::DateTime<chrono::Utc>>,
    pub discarding_id: Option<Uuid>,
    pub antecedent_id: Option<Uuid>,
    pub antecedent_hash: Option<String>,
    pub hash: Option<String>,
    pub hash_alg: Option<String>,
}
