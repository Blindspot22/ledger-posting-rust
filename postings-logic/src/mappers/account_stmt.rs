use postings_api::domain::account_stmt::AccountStmt as AccountStmtBO;
use postings_db::models::account_stmt::AccountStmt as AccountStmtModel;
use postings_api::domain::financial_stmt::FinancialStmt;
use uuid::Uuid;

pub struct AccountStmtMapper;

impl AccountStmtMapper {
    pub fn to_bo(model: AccountStmtModel, account_bo: postings_api::domain::ledger_account::LedgerAccount, posting_bo: Option<postings_api::domain::posting::Posting>, youngest_pst_bo: Option<postings_api::domain::posting_trace::PostingTrace>, latest_pst_bo: Option<postings_api::domain::posting_trace::PostingTrace>) -> AccountStmtBO {
        AccountStmtBO {
            financial_stmt: FinancialStmt {
                id: Uuid::parse_str(&model.id).unwrap(),
                posting: posting_bo,
                pst_time: model.pst_time,
                stmt_status: match model.stmt_status {
                    postings_db::models::stmt_status::StmtStatus::Simulated => postings_api::domain::stmt_status::StmtStatus::SIMULATED,
                    postings_db::models::stmt_status::StmtStatus::Closed => postings_api::domain::stmt_status::StmtStatus::CLOSED,
                },
                latest_pst: latest_pst_bo,
                stmt_seq_nbr: model.stmt_seq_nbr,
            },
            account: account_bo,
            youngest_pst: youngest_pst_bo,
            total_debit: model.total_debit,
            total_credit: model.total_credit,
        }
    }

    pub fn from_bo(bo: AccountStmtBO) -> AccountStmtModel {
        AccountStmtModel {
            id: bo.financial_stmt.id.to_string(),
            account_id: bo.account.named.id.to_string(),
            youngest_pst_id: bo.youngest_pst.map(|p| p.id.to_string()),
            total_debit: bo.total_debit,
            total_credit: bo.total_credit,
            posting_id: bo.financial_stmt.posting.map(|p| p.id.to_string()),
            pst_time: bo.financial_stmt.pst_time,
            stmt_status: match bo.financial_stmt.stmt_status {
                postings_api::domain::stmt_status::StmtStatus::SIMULATED => postings_db::models::stmt_status::StmtStatus::Simulated,
                postings_api::domain::stmt_status::StmtStatus::CLOSED => postings_db::models::stmt_status::StmtStatus::Closed,
            },
            latest_pst_id: bo.financial_stmt.latest_pst.map(|p| p.id.to_string()),
            stmt_seq_nbr: bo.financial_stmt.stmt_seq_nbr,
        }
    }
}
