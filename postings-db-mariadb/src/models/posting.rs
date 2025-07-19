use uuid::Uuid;
use sqlx::FromRow;
use postings_db::models::posting_type::PostingType;
use postings_db::models::posting_status::PostingStatus;
use postings_db::models::posting::Posting;

#[derive(Debug, Clone, FromRow, PartialEq)]
pub struct PostingDb {
    pub id: String,
    pub record_user: Vec<u8>,
    pub record_time: chrono::DateTime<chrono::Utc>,
    pub opr_id: Vec<u8>,
    pub opr_time: chrono::DateTime<chrono::Utc>,
    pub opr_type: Vec<u8>,
    pub opr_details: Option<Vec<u8>>,
    pub opr_src: Option<Vec<u8>>,
    pub pst_time: chrono::DateTime<chrono::Utc>,
    pub pst_type: String,
    pub pst_status: String,
    pub ledger_id: String,
    pub val_time: Option<chrono::DateTime<chrono::Utc>>,
    pub discarded_id: Option<String>,
    pub discarded_time: Option<chrono::DateTime<chrono::Utc>>,
    pub discarding_id: Option<String>,
    pub antecedent_id: Option<String>,
    pub antecedent_hash: Option<Vec<u8>>,
    pub hash: Option<Vec<u8>>,
}

impl From<PostingDb> for Posting {
    fn from(p: PostingDb) -> Self {
        Self {
            id: Uuid::parse_str(&p.id).unwrap(),
            record_user: p.record_user.try_into().unwrap_or([0u8; 34]),
            record_time: p.record_time,
            opr_id: p.opr_id.try_into().unwrap_or([0u8; 34]),
            opr_time: p.opr_time,
            opr_type: p.opr_type.try_into().unwrap_or([0u8; 34]),
            opr_details: p.opr_details.map(|v| v.try_into().unwrap_or([0u8; 34])),
            opr_src: p.opr_src.map(|v| v.try_into().unwrap_or([0u8; 34])),
            pst_time: p.pst_time,
            pst_type: match p.pst_type.as_str() {
                "BUSI_TX" => PostingType::BusiTx,
                "ADJ_TX" => PostingType::AdjTx,
                "BAL_STMT" => PostingType::BalStmt,
                "PNL_STMT" => PostingType::PnlStmt,
                "BS_STMT" => PostingType::BsStmt,
                "LDG_CLSNG" => PostingType::LdgClsng,
                _ => PostingType::Unknown,
            },
            pst_status: match p.pst_status.as_str() {
                "DEFERRED" => PostingStatus::Deferred,
                "POSTED" => PostingStatus::Posted,
                "PROPOSED" => PostingStatus::Proposed,
                "SIMULATED" => PostingStatus::Simulated,
                "TAX" => PostingStatus::Tax,
                "UNPOSTED" => PostingStatus::Unposted,
                "CANCELLED" => PostingStatus::Cancelled,
                _ => PostingStatus::Other,
            },
            ledger_id: Uuid::parse_str(&p.ledger_id).unwrap(),
            val_time: p.val_time,
            discarded_id: p.discarded_id.map(|s| Uuid::parse_str(&s).unwrap()),
            discarded_time: p.discarded_time,
            discarding_id: p.discarding_id.map(|s| Uuid::parse_str(&s).unwrap()),
            antecedent_id: p.antecedent_id.map(|s| Uuid::parse_str(&s).unwrap()),
            antecedent_hash: p.antecedent_hash.map(|v| v.try_into().unwrap_or([0u8; 34])),
            hash: p.hash.map(|v| v.try_into().unwrap_or([0u8; 34])),
        }
    }
}

impl From<Posting> for PostingDb {
    fn from(p: Posting) -> Self {
        Self {
            id: p.id.to_string(),
            record_user: p.record_user.to_vec(),
            record_time: p.record_time,
            opr_id: p.opr_id.to_vec(),
            opr_time: p.opr_time,
            opr_type: p.opr_type.to_vec(),
            opr_details: p.opr_details.map(|v| v.to_vec()),
            opr_src: p.opr_src.map(|v| v.to_vec()),
            pst_time: p.pst_time,
            pst_type: match p.pst_type {
                PostingType::BusiTx => "BUSI_TX".to_string(),
                PostingType::AdjTx => "ADJ_TX".to_string(),
                PostingType::BalStmt => "BAL_STMT".to_string(),
                PostingType::PnlStmt => "PNL_STMT".to_string(),
                PostingType::BsStmt => "BS_STMT".to_string(),
                PostingType::LdgClsng => "LDG_CLSNG".to_string(),
                PostingType::Unknown => "UNKNOWN".to_string(),
            },
            pst_status: match p.pst_status {
                PostingStatus::Deferred => "DEFERRED".to_string(),
                PostingStatus::Posted => "POSTED".to_string(),
                PostingStatus::Proposed => "PROPOSED".to_string(),
                PostingStatus::Simulated => "SIMULATED".to_string(),
                PostingStatus::Tax => "TAX".to_string(),
                PostingStatus::Unposted => "UNPOSTED".to_string(),
                PostingStatus::Cancelled => "CANCELLED".to_string(),
                PostingStatus::Other => "OTHER".to_string(),
            },
            ledger_id: p.ledger_id.to_string(),
            val_time: p.val_time,
            discarded_id: p.discarded_id.map(|uuid| uuid.to_string()),
            discarded_time: p.discarded_time,
            discarding_id: p.discarding_id.map(|uuid| uuid.to_string()),
            antecedent_id: p.antecedent_id.map(|uuid| uuid.to_string()),
            antecedent_hash: p.antecedent_hash.map(|v| v.to_vec()),
            hash: p.hash.map(|v| v.to_vec()),
        }
    }
}
