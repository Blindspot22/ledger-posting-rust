use uuid::Uuid;
use sqlx::FromRow;
use bigdecimal::BigDecimal;
use postings_db::models::posting_type::PostingType;
use postings_db::models::posting_status::PostingStatus;
use postings_db::models::posting_line::PostingLine;

#[derive(Debug, Clone, FromRow, PartialEq)]
pub struct PostingLineDb {
    pub id: String,
    pub account_id: String,
    pub debit_amount: BigDecimal,
    pub credit_amount: BigDecimal,
    pub details: Option<Vec<u8>>,
    pub src_account: Option<Vec<u8>>,
    pub base_line: Option<String>,
    pub sub_opr_src_id: Option<Vec<u8>>,
    pub record_time: chrono::DateTime<chrono::Utc>,
    pub opr_id: Vec<u8>,
    pub opr_src: Option<Vec<u8>>,
    pub pst_time: chrono::DateTime<chrono::Utc>,
    pub pst_type: String,
    pub pst_status: String,
    pub hash: Option<Vec<u8>>,
    pub discarded_time: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<PostingLineDb> for PostingLine {
    fn from(p: PostingLineDb) -> Self {
        Self {
            id: Uuid::parse_str(&p.id).unwrap(),
            account_id: Uuid::parse_str(&p.account_id).unwrap(),
            debit_amount: p.debit_amount,
            credit_amount: p.credit_amount,
            details: p.details.map(|v| v.try_into().unwrap_or([0u8; 34])),
            src_account: p.src_account.map(|v| v.try_into().unwrap_or([0u8; 34])),
            base_line: p.base_line.map(|s| Uuid::parse_str(&s).unwrap()),
            sub_opr_src_id: p.sub_opr_src_id.map(|v| v.try_into().unwrap_or([0u8; 34])),
            record_time: p.record_time,
            opr_id: p.opr_id.try_into().unwrap_or([0u8; 34]),
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
            hash: p.hash.map(|v| v.try_into().unwrap_or([0u8; 34])),
            discarded_time: p.discarded_time,
        }
    }
}

impl From<PostingLine> for PostingLineDb {
    fn from(p: PostingLine) -> Self {
        Self {
            id: p.id.to_string(),
            account_id: p.account_id.to_string(),
            debit_amount: p.debit_amount,
            credit_amount: p.credit_amount,
            details: p.details.map(|v| v.to_vec()),
            src_account: p.src_account.map(|v| v.to_vec()),
            base_line: p.base_line.map(|uuid| uuid.to_string()),
            sub_opr_src_id: p.sub_opr_src_id.map(|v| v.to_vec()),
            record_time: p.record_time,
            opr_id: p.opr_id.to_vec(),
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
            hash: p.hash.map(|v| v.to_vec()),
            discarded_time: p.discarded_time,
        }
    }
}