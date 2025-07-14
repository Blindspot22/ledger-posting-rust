use postings_api::domain::posting::Posting as PostingBO;
use postings_db::models::posting::Posting as PostingModel;
use postings_api::domain::hash_record::HashRecord;

pub struct PostingMapper;

impl PostingMapper {
    pub fn to_bo(model: PostingModel, ledger_bo: postings_api::domain::ledger::Ledger, lines_bo: Vec<postings_api::domain::posting_line::PostingLine>) -> PostingBO {
        PostingBO {
            id: model.id,
            record_user: model.record_user,
            record_time: model.record_time,
            opr_id: model.opr_id,
            opr_time: model.opr_time,
            opr_type: model.opr_type,
            opr_details: model.opr_details,
            opr_src: model.opr_src,
            pst_time: model.pst_time,
            pst_type: match model.pst_type {
                postings_db::models::posting_type::PostingType::BusiTx => postings_api::domain::posting_type::PostingType::BusiTx,
                postings_db::models::posting_type::PostingType::AdjTx => postings_api::domain::posting_type::PostingType::AdjTx,
                postings_db::models::posting_type::PostingType::BalStmt => postings_api::domain::posting_type::PostingType::BalStmt,
                postings_db::models::posting_type::PostingType::PnlStmt => postings_api::domain::posting_type::PostingType::PnLStmt,
                postings_db::models::posting_type::PostingType::BsStmt => postings_api::domain::posting_type::PostingType::BsStmt,
                postings_db::models::posting_type::PostingType::LdgClsng => postings_api::domain::posting_type::PostingType::LdgClsng,
            },
            pst_status: match model.pst_status {
                postings_db::models::posting_status::PostingStatus::Deferred => postings_api::domain::posting_status::PostingStatus::Deferred,
                postings_db::models::posting_status::PostingStatus::Posted => postings_api::domain::posting_status::PostingStatus::Posted,
                postings_db::models::posting_status::PostingStatus::Proposed => postings_api::domain::posting_status::PostingStatus::Proposed,
                postings_db::models::posting_status::PostingStatus::Simulated => postings_api::domain::posting_status::PostingStatus::Simulated,
                postings_db::models::posting_status::PostingStatus::Tax => postings_api::domain::posting_status::PostingStatus::Tax,
                postings_db::models::posting_status::PostingStatus::Unposted => postings_api::domain::posting_status::PostingStatus::Unposted,
                postings_db::models::posting_status::PostingStatus::Cancelled => postings_api::domain::posting_status::PostingStatus::Cancelled,
                postings_db::models::posting_status::PostingStatus::Other => postings_api::domain::posting_status::PostingStatus::Other,
            },
            ledger: ledger_bo,
            val_time: model.val_time,
            lines: lines_bo,
            discarded_id: model.discarded_id,
            discarded_time: model.discarded_time,
            discarding_id: model.discarding_id,
            hash_record: HashRecord {
                antecedent_id: model.antecedent_id,
                antecedent_hash: model.antecedent_hash,
                hash: model.hash,
                hash_alg: model.hash_alg,
            },
        }
    }

    pub fn to_model(bo: PostingBO) -> PostingModel {
        PostingModel {
            id: bo.id,
            record_user: bo.record_user,
            record_time: bo.record_time,
            opr_id: bo.opr_id,
            opr_time: bo.opr_time,
            opr_type: bo.opr_type,
            opr_details: bo.opr_details,
            opr_src: bo.opr_src,
            pst_time: bo.pst_time,
            pst_type: match bo.pst_type {
                postings_api::domain::posting_type::PostingType::BusiTx => postings_db::models::posting_type::PostingType::BusiTx,
                postings_api::domain::posting_type::PostingType::AdjTx => postings_db::models::posting_type::PostingType::AdjTx,
                postings_api::domain::posting_type::PostingType::BalStmt => postings_db::models::posting_type::PostingType::BalStmt,
                postings_api::domain::posting_type::PostingType::PnLStmt => postings_db::models::posting_type::PostingType::PnlStmt,
                postings_api::domain::posting_type::PostingType::BsStmt => postings_db::models::posting_type::PostingType::BsStmt,
                postings_api::domain::posting_type::PostingType::LdgClsng => postings_db::models::posting_type::PostingType::LdgClsng,
            },
            pst_status: match bo.pst_status {
                postings_api::domain::posting_status::PostingStatus::Deferred => postings_db::models::posting_status::PostingStatus::Deferred,
                postings_api::domain::posting_status::PostingStatus::Posted => postings_db::models::posting_status::PostingStatus::Posted,
                postings_api::domain::posting_status::PostingStatus::Proposed => postings_db::models::posting_status::PostingStatus::Proposed,
                postings_api::domain::posting_status::PostingStatus::Simulated => postings_db::models::posting_status::PostingStatus::Simulated,
                postings_api::domain::posting_status::PostingStatus::Tax => postings_db::models::posting_status::PostingStatus::Tax,
                postings_api::domain::posting_status::PostingStatus::Unposted => postings_db::models::posting_status::PostingStatus::Unposted,
                postings_api::domain::posting_status::PostingStatus::Cancelled => postings_db::models::posting_status::PostingStatus::Cancelled,
                postings_api::domain::posting_status::PostingStatus::Other => postings_db::models::posting_status::PostingStatus::Other,
            },
            ledger_id: bo.ledger.named.id,
            val_time: bo.val_time,
            discarded_id: bo.discarded_id,
            discarded_time: bo.discarded_time,
            discarding_id: bo.discarding_id,
            antecedent_id: bo.hash_record.antecedent_id,
            antecedent_hash: bo.hash_record.antecedent_hash,
            hash: bo.hash_record.hash,
            hash_alg: bo.hash_record.hash_alg,
        }
    }
}
