use postings_api::domain::posting_line::PostingLine as PostingLineBO;
use postings_db::models::posting_line::PostingLine as PostingLineModel;

pub struct PostingLineMapper;

impl PostingLineMapper {
    pub fn to_bo(model: PostingLineModel, account_bo: postings_api::domain::ledger_account::LedgerAccount) -> PostingLineBO {
        PostingLineBO {
            id: model.id,
            account: account_bo,
            debit_amount: model.debit_amount,
            credit_amount: model.credit_amount,
            details : model.details,
            src_account: model.src_account,
            base_line: model.base_line,
            sub_opr_src_id: model.sub_opr_src_id,
            record_time: model.record_time,
            opr_id: model.opr_id,
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
            hash: model.hash,
            additional_information: None, // Not in DB model
            discarded_time: model.discarded_time,
        }
    }

    pub fn from_bo(bo: PostingLineBO) -> PostingLineModel {
        PostingLineModel {
            id: bo.id,
            account_id: bo.account.named.id,
            debit_amount: bo.debit_amount,
            credit_amount: bo.credit_amount,
            details: bo.details,
            src_account: bo.src_account,
            base_line: bo.base_line,
            sub_opr_src_id: bo.sub_opr_src_id,
            record_time: bo.record_time,
            opr_id: bo.opr_id,
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
            hash: bo.hash,
            discarded_time: bo.discarded_time,
        }
    }
}
