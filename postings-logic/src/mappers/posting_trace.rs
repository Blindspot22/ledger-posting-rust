use postings_api::domain::posting_trace::PostingTrace as PostingTraceBO;
use postings_db::models::posting_trace::PostingTrace as PostingTraceModel;
use uuid::Uuid;

pub struct PostingTraceMapper;

impl PostingTraceMapper {
    pub fn to_bo(model: PostingTraceModel, account_bo: postings_api::domain::ledger_account::LedgerAccount) -> PostingTraceBO {
        PostingTraceBO {
            id: Uuid::parse_str(&model.id).unwrap(),
            tgt_pst_id: Uuid::parse_str(&model.tgt_pst_id).unwrap(),
            src_pst_time: model.src_pst_time,
            src_pst_id: Uuid::parse_str(&model.src_pst_id).unwrap(),
            src_opr_id: model.src_opr_id,
            account: account_bo,
            debit_amount: model.debit_amount,
            credit_amount: model.credit_amount,
            src_pst_hash: model.src_pst_hash,
        }
    }

    pub fn from_bo(bo: PostingTraceBO) -> PostingTraceModel {
        PostingTraceModel {
            id: bo.id.to_string(),
            tgt_pst_id: bo.tgt_pst_id.to_string(),
            src_pst_time: bo.src_pst_time,
            src_pst_id: bo.src_pst_id.to_string(),
            src_opr_id: bo.src_opr_id,
            account_id: bo.account.named.id.to_string(),
            debit_amount: bo.debit_amount,
            credit_amount: bo.credit_amount,
            src_pst_hash: bo.src_pst_hash,
        }
    }
}
