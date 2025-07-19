use uuid::Uuid;
use sqlx::FromRow;
use postings_db::models::posting_type::PostingType;
use postings_db::models::posting_status::PostingStatus;
use postings_db::models::posting::Posting;

#[derive(Debug, Clone, FromRow, PartialEq)]
pub struct PostingDb {
    pub id: Uuid,
    pub record_user: [u8; 34],
    pub record_time: chrono::DateTime<chrono::Utc>,
    pub opr_id: [u8; 34],
    pub opr_time: chrono::DateTime<chrono::Utc>,
    pub opr_type: [u8; 34],
    pub opr_details: Option<[u8; 34]>,
    pub opr_src: Option<[u8; 34]>,
    pub pst_time: chrono::DateTime<chrono::Utc>,
    pub pst_type: PostingType,
    pub pst_status: PostingStatus,
    pub ledger_id: Uuid,
    pub val_time: Option<chrono::DateTime<chrono::Utc>>,
    pub discarded_id: Option<Uuid>,
    pub discarded_time: Option<chrono::DateTime<chrono::Utc>>,
    pub discarding_id: Option<Uuid>,
    pub antecedent_id: Option<Uuid>,
    pub antecedent_hash: Option<[u8; 34]>,
    pub hash: Option<[u8; 34]>,
}

impl From<PostingDb> for Posting {
    fn from(p: PostingDb) -> Self {
        Self {
            id: p.id,
            record_user: p.record_user.try_into().unwrap_or_default(),
            record_time: p.record_time,
            opr_id: p.opr_id.try_into().unwrap_or_default(),
            opr_time: p.opr_time,
            opr_type: p.opr_type.try_into().unwrap_or_default(),
            opr_details: p.opr_details.map(|v| v.try_into().unwrap_or_default()),
            opr_src: p.opr_src.map(|v| v.try_into().unwrap_or_default()),
            pst_time: p.pst_time,
            pst_type: p.pst_type,
            pst_status: p.pst_status,
            ledger_id: p.ledger_id,
            val_time: p.val_time,
            discarded_id: p.discarded_id,
            discarded_time: p.discarded_time,
            discarding_id: p.discarding_id,
            antecedent_id: p.antecedent_id,
            antecedent_hash: p.antecedent_hash,
            hash: p.hash,
        }
    }
}
