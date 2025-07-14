use std::sync::Arc;
use postings_db::repositories::chart_of_account_repository::ChartOfAccountRepository;
use postings_db::repositories::ledger_account_repository::LedgerAccountRepository;
use postings_db::repositories::ledger_repository::LedgerRepository;
use postings_db::repositories::posting_repository::PostingRepository;
use postings_db::repositories::account_stmt_repository::AccountStmtRepository;
use postings_db::repositories::posting_line_repository::PostingLineRepository;
use postings_db::repositories::posting_trace_repository::PostingTraceRepository;
use postings_api::domain::chart_of_account::ChartOfAccount;
use postings_api::domain::ledger::Ledger;
use postings_api::domain::ledger_account::LedgerAccount;
use postings_api::ServiceError;
use postings_db::DbError;

pub struct SharedService {
    pub coa_repo: Arc<dyn ChartOfAccountRepository + Send + Sync>,
    pub ledger_repo: Arc<dyn LedgerRepository + Send + Sync>,
    pub ledger_account_repo: Arc<dyn LedgerAccountRepository + Send + Sync>,
    pub posting_repo: Arc<dyn PostingRepository + Send + Sync>,
    pub stmt_repo: Arc<dyn AccountStmtRepository + Send + Sync>,
    pub line_repo: Arc<dyn PostingLineRepository + Send + Sync>,
    pub trace_repo: Arc<dyn PostingTraceRepository + Send + Sync>,
}

impl SharedService {
    pub fn new(
        coa_repo: Arc<dyn ChartOfAccountRepository + Send + Sync>,
        ledger_repo: Arc<dyn LedgerRepository + Send + Sync>,
        ledger_account_repo: Arc<dyn LedgerAccountRepository + Send + Sync>,
        posting_repo: Arc<dyn PostingRepository + Send + Sync>,
        stmt_repo: Arc<dyn AccountStmtRepository + Send + Sync>,
        line_repo: Arc<dyn PostingLineRepository + Send + Sync>,
        trace_repo: Arc<dyn PostingTraceRepository + Send + Sync>,
    ) -> Self {
        Self {
            coa_repo,
            ledger_repo,
            ledger_account_repo,
            posting_repo,
            stmt_repo,
            line_repo,
            trace_repo,
        }
    }

    pub async fn load_coa(&self, chart_of_account: &ChartOfAccount) -> Result<postings_db::models::chart_of_account::ChartOfAccount, ServiceError> {
        self.coa_repo
            .find_by_id(chart_of_account.named.id)
            .await
            .map_err(|_| ServiceError::Db)?
            .ok_or(ServiceError::ChartOfAccountNotFound)
    }
    
    pub async fn load_ledger(&self, ledger: &Ledger) -> Result<postings_db::models::ledger::Ledger, ServiceError> {
        self.ledger_repo
            .find_by_id(ledger.named.id)
            .await
            .map_err(|_| ServiceError::Db)?
            .ok_or(ServiceError::LedgerNotFound)
    }

    pub async fn load_ledger_account(&self, ledger_account: &LedgerAccount) -> Result<Option<postings_db::models::ledger_account::LedgerAccount>, ServiceError> {
        match self.ledger_account_repo
            .find_by_id(ledger_account.named.id)
            .await
        {
            Ok(account) => Ok(Some(account)),
            Err(DbError::NotFound) => Ok(None),
            Err(e) => {
                log::error!("Database error loading ledger account: {e:?}");
                Err(ServiceError::Db)
            }
        }
    }
}
