use async_trait::async_trait;
use postings_api::domain::account_stmt::AccountStmt;
use postings_api::domain::ledger_account::LedgerAccount;
use postings_api::service::account_stmt_service::AccountStmtService;
use postings_api::ServiceError;
use crate::services::shared_service::SharedService;
use chrono::{DateTime, Utc};
use postings_db::models::stmt_status::StmtStatus;
use uuid::Uuid;
use bigdecimal::BigDecimal;
use log::info;
use postings_db::models::posting_line::PostingLine;
use postings_db::models::posting_trace::PostingTrace;

pub struct AccountStmtServiceImpl {
    shared: SharedService,
}

impl AccountStmtServiceImpl {
    pub fn new(shared: SharedService) -> Self {
        Self { shared }
    }

    async fn stmt(&self, ledger_account: LedgerAccount, ref_time: DateTime<Utc>) -> Result<AccountStmt, ServiceError> {
        info!("Generating statement for account: {} at time: {}", ledger_account.named.id, ref_time);
        let account_model = self.shared.load_ledger_account(&ledger_account).await
            .map_err(|e| {
                info!("Error loading ledger account: {e:?}");
                ServiceError::Db
            })?
            .ok_or_else(|| {
                info!("Ledger account not found");
                ServiceError::LedgerAccountNotFound
            })?;

        info!("Loaded account model: {}", account_model.id);
        let last_closed_stmt = self.shared.stmt_repo
            .find_first_by_account_and_status_and_pst_time_less_than_ordered(&account_model.id, StmtStatus::Closed, ref_time)
            .await
            .map_err(|e| {
                info!("Error finding last closed statement: {e:?}");
                ServiceError::Db
            })?;

        let (mut stmt, posting_lines) = if let Some(last_stmt) = last_closed_stmt {
            info!("Found last closed statement: {}", last_stmt.id);
            let lines = self.shared.line_repo
                .find_by_account_and_pst_time_less_than_equal(&account_model.id, ref_time)
                .await
                .map_err(|e| {
                    info!("Error finding posting lines for existing statement: {e:?}");
                    ServiceError::Db
                })?;
            (last_stmt, lines)
        } else {
            info!("No closed statement found, creating new simulated statement");
            let new_stmt = postings_db::models::account_stmt::AccountStmt {
                id: Uuid::new_v4().to_string(),
                account_id: account_model.id.clone(),
                youngest_pst_id: None,
                total_debit: BigDecimal::from(0),
                total_credit: BigDecimal::from(0),
                posting_id: None,
                pst_time: ref_time,
                stmt_status: StmtStatus::Simulated,
                latest_pst_id: None,
                stmt_seq_nbr: 0,
            };
            let lines = self.shared.line_repo
                .find_by_account_and_pst_time_less_than_equal(&account_model.id, ref_time)
                .await
                .map_err(|e| {
                    info!("Error finding posting lines for new statement: {e:?}");
                    ServiceError::Db
                })?;
            (new_stmt, lines)
        };

        info!("Found {} posting lines", posting_lines.len());
        for line in posting_lines {
            self.refresh_statement(&mut stmt, &line).await
                .map_err(|e| {
                    info!("Error refreshing statement with line {}: {e:?}", line.id);
                    e
                })?;
        }

        // Simplified mapping
        Ok(AccountStmt {
            financial_stmt: postings_api::domain::financial_stmt::FinancialStmt {
                id: Uuid::parse_str(&stmt.id).unwrap(),
                posting: None,
                pst_time: stmt.pst_time,
                stmt_status: postings_api::domain::stmt_status::StmtStatus::SIMULATED,
                latest_pst: None,
                stmt_seq_nbr: stmt.stmt_seq_nbr,
            },
            account: ledger_account,
            youngest_pst: None,
            total_debit: stmt.total_debit,
            total_credit: stmt.total_credit,
        })
    }

    async fn refresh_statement(&self, stmt: &mut postings_db::models::account_stmt::AccountStmt, line: &PostingLine) -> Result<(), ServiceError> {
        let trace = self.create_posting_trace(stmt, line);
        info!("Created posting trace: {}", trace.id);
        
        if stmt.youngest_pst_id.is_none() { // Simplified logic
            stmt.youngest_pst_id = Some(trace.id.clone());
        }
        stmt.latest_pst_id = Some(trace.id.clone());
        stmt.total_debit += line.debit_amount.clone();
        stmt.total_credit += line.credit_amount.clone();
        
        self.shared.trace_repo.save(trace).await.map_err(|e| {
            info!("Error saving posting trace: {e:?}");
            ServiceError::Db
        })?;
        Ok(())
    }

    fn create_posting_trace(&self, stmt: &postings_db::models::account_stmt::AccountStmt, line: &PostingLine) -> PostingTrace {
        PostingTrace {
            id: Uuid::new_v4().to_string(),
            tgt_pst_id: stmt.id.clone(),
            src_pst_time: line.pst_time,
            src_pst_id: line.id.clone(),
            src_opr_id: line.opr_id.clone(),
            account_id: stmt.account_id.clone(),
            debit_amount: line.debit_amount.clone(),
            credit_amount: line.credit_amount.clone(),
            src_pst_hash: line.hash.clone(),
        }
    }
}

#[async_trait]
impl AccountStmtService for AccountStmtServiceImpl {
    async fn read_stmt(&self, ledger_account: LedgerAccount, ref_time: DateTime<Utc>) -> Result<AccountStmt, ServiceError> {
        self.stmt(ledger_account, ref_time).await
    }

    async fn create_stmt(&self, ledger_account: LedgerAccount, ref_time: DateTime<Utc>) -> Result<AccountStmt, ServiceError> {
        let stmt = self.stmt(ledger_account, ref_time).await?;
        // self.shared.stmt_repo.save(stmt).await.map_err(|_| ServiceError::Db)?; // stmt is BO, needs mapping
        Ok(stmt)
    }

    async fn close_stmt(&self, _stmt: AccountStmt) -> Result<AccountStmt, ServiceError> {
        unimplemented!()
    }
}