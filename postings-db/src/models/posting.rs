use chrono::{DateTime, Utc};
use uuid::Uuid;
use sqlx::FromRow;
use crate::models::posting_type::PostingType;
use crate::models::posting_status::PostingStatus;

#[derive(Debug, Clone, FromRow, PartialEq)]
pub struct Posting {
    pub id: Uuid,
    /// User that recorded the posting. It is a 32-byte hash.
    pub record_user: [u8; 34],
    pub record_time: DateTime<Utc>,
    /// Operation ID. It is a 32-byte hash of the lowercase string of the original information.
    pub opr_id: [u8; 34],
    pub opr_time: DateTime<Utc>,
    /// Operation Type. It is a 32-byte hash of the lowercase string of the original information.
    pub opr_type: [u8; 34],
    /// Operation Details. It is a 32-byte hash of the lowercase string of the original information.
    pub opr_details: Option<[u8; 34]>,
    /// Operation Source. It is a 32-byte hash of the lowercase string of the original information.
    pub opr_src: Option<[u8; 34]>,
    pub pst_time: DateTime<Utc>,
    pub pst_type: PostingType,
    pub pst_status: PostingStatus,
    pub ledger_id: Uuid,
    pub val_time: Option<DateTime<Utc>>,
    pub discarded_id: Option<Uuid>,
    pub discarded_time: Option<DateTime<Utc>>,
    pub discarding_id: Option<Uuid>,
    pub antecedent_id: Option<Uuid>,
    /// Multihash of the antecedent posting.
    pub antecedent_hash: Option<[u8; 34]>,
    /// Multihash of the posting.
    pub hash: Option<[u8; 34]>,
}
