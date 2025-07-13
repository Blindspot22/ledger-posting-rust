use async_trait::async_trait;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use log::{error, info};
use uuid::Uuid;

use postings_api::domain::account_stmt::AccountStmt;
use postings_api::domain::financial_stmt::FinancialStmt;
use postings_api::domain::ledger_account::LedgerAccount;
use postings_api::domain::posting_status::PostingStatus;
use postings_api::domain::posting_type::PostingType;
use postings_api::service::account_stmt_service::AccountStmtService;
use postings_api::ServiceError;
use postings_db::models::posting_line::PostingLine;
use postings_db::models::posting_trace::PostingTrace;
use postings_db::models::stmt_status::StmtStatus;

use crate::hash_utils::hash_serialize;
use crate::mappers::account_stmt::AccountStmtMapper;
use crate::mappers::ledger::LedgerMapper;
use crate::mappers::posting::PostingMapper;
use crate::mappers::posting_trace::PostingTraceMapper;
use crate::services::shared_service::SharedService;

pub struct AccountStmtServiceImpl {
    shared: SharedService,
}

impl AccountStmtServiceImpl {
    pub fn new(shared: SharedService) -> Self {
        Self { shared }
    }

    async fn stmt(
        &self,
        ledger_account: LedgerAccount,
        ref_time: DateTime<Utc>,
    ) -> Result<AccountStmt, ServiceError> {
        info!(
            "Generating statement for account: {} at time: {}",
            ledger_account.named.id, ref_time
        );
        let account_model = self
            .shared
            .load_ledger_account(&ledger_account)
            .await
            .map_err(|e| {
                info!("Error loading ledger account: {e:?}");
                ServiceError::Db
            })?
            .ok_or_else(|| {
                info!("Ledger account not found");
                ServiceError::LedgerAccountNotFound
            })?;

        info!("Loaded account model: {}", account_model.id);
        let last_closed_stmt = self
            .shared
            .stmt_repo
            .find_first_by_account_and_status_and_pst_time_less_than_ordered(
                &account_model.id,
                StmtStatus::Closed,
                ref_time,
            )
            .await
            .map_err(|e| {
                info!("Error finding last closed statement: {e:?}");
                ServiceError::Db
            })?;

        let (mut stmt, posting_lines) = if let Some(last_stmt) = last_closed_stmt {
            info!("Found last closed statement: {}", last_stmt.id);
            let lines = self
                .shared
                .line_repo
                .find_by_account_and_pst_time_between(
                    &account_model.id,
                    last_stmt.pst_time,
                    ref_time,
                )
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
            let lines = self
                .shared
                .line_repo
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
            self.refresh_statement(&mut stmt, &line)
                .await
                .map_err(|e| {
                    info!("Error refreshing statement with line {}: {e:?}", line.id);
                    e
                })?;
        }

        let youngest_pst_bo = if let Some(id) = &stmt.youngest_pst_id {
            self.shared
                .trace_repo
                .find_by_id(id)
                .await
                .map_err(|_| ServiceError::Db)?
                .map(|tm| PostingTraceMapper::to_bo(tm, ledger_account.clone()))
        } else {
            None
        };
        let latest_pst_bo = if let Some(id) = &stmt.latest_pst_id {
            self.shared
                .trace_repo
                .find_by_id(id)
                .await
                .map_err(|_| ServiceError::Db)?
                .map(|tm| PostingTraceMapper::to_bo(tm, ledger_account.clone()))
        } else {
            None
        };
        let posting_bo = if let Some(id) = &stmt.posting_id {
            self.shared
                .posting_repo
                .find_by_id(id)
                .await
                .map_err(|_| ServiceError::Db)?
                .map(|pm| {
                    // This mapping is incomplete as it requires more context (ledger, lines, etc.)
                    // For now, we create a simplified Posting BO, as the full details are not needed for the statement view.
                    let ledger_bo = ledger_account.ledger.clone();
                    let opr_details = "".to_string(); // Not fetching details for this view
                    PostingMapper::to_bo(pm, ledger_bo, vec![], opr_details)
                })
        } else {
            None
        };

        Ok(AccountStmt {
            financial_stmt: FinancialStmt {
                id: Uuid::parse_str(&stmt.id).unwrap(),
                posting: posting_bo,
                pst_time: stmt.pst_time,
                stmt_status: match stmt.stmt_status {
                    StmtStatus::Simulated => postings_api::domain::stmt_status::StmtStatus::SIMULATED,
                    StmtStatus::Closed => postings_api::domain::stmt_status::StmtStatus::CLOSED,
                },
                latest_pst: latest_pst_bo,
                stmt_seq_nbr: stmt.stmt_seq_nbr,
            },
            account: ledger_account,
            youngest_pst: youngest_pst_bo,
            total_debit: stmt.total_debit,
            total_credit: stmt.total_credit,
        })
    }

    async fn refresh_statement(
        &self,
        stmt: &mut postings_db::models::account_stmt::AccountStmt,
        line: &PostingLine,
    ) -> Result<(), ServiceError> {
        let trace = self.create_posting_trace(stmt, line);
        info!("Created posting trace: {}", trace.id);

        if stmt.youngest_pst_id.is_none() {
            // Simplified logic
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

    fn create_posting_trace(
        &self,
        stmt: &postings_db::models::account_stmt::AccountStmt,
        line: &PostingLine,
    ) -> PostingTrace {
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
    async fn read_stmt(
        &self,
        ledger_account: LedgerAccount,
        ref_time: DateTime<Utc>,
    ) -> Result<AccountStmt, ServiceError> {
        self.stmt(ledger_account, ref_time).await
    }

    async fn create_stmt(
        &self,
        ledger_account: LedgerAccount,
        ref_time: DateTime<Utc>,
    ) -> Result<AccountStmt, ServiceError> {
        let stmt_bo = self.stmt(ledger_account, ref_time).await?;
        let stmt_model = AccountStmtMapper::from_bo(stmt_bo.clone());
        self.shared.stmt_repo.save(stmt_model).await.map_err(|e| {
            error!("Failed to save statement: {:?}", e);
            ServiceError::Db
        })?;
        Ok(stmt_bo)
    }

    async fn close_stmt(&self, stmt: AccountStmt) -> Result<AccountStmt, ServiceError> {
        let mut stmt_model = self
            .shared
            .stmt_repo
            .find_by_id(&stmt.financial_stmt.id.to_string())
            .await
            .map_err(|_| ServiceError::Db)?
            .ok_or(ServiceError::StatementNotFound)?;

        if stmt_model.stmt_status == StmtStatus::Closed {
            return Err(ServiceError::StatementAlreadyClosed);
        }

        let ledger_model = self
            .shared
            .ledger_repo
            .find_by_id(&stmt.account.ledger.named.id.to_string())
            .await
            .map_err(|_| ServiceError::Db)?
            .unwrap();
        let coa_bo = self
            .shared
            .coa_repo
            .find_by_id(&ledger_model.coa_id)
            .await
            .map_err(|_| ServiceError::Db)?
            .map(crate::mappers::chart_of_account::ChartOfAccountMapper::to_bo)
            .unwrap();
        let ledger_bo = LedgerMapper::to_bo(ledger_model, coa_bo);

        let mut closing_posting = postings_api::domain::posting::Posting {
            id: Uuid::new_v4(),
            record_user: "system".to_string(),
            record_time: Utc::now(),
            opr_id: format!("stmt-close-{}", stmt.financial_stmt.id),
            opr_time: Utc::now(),
            opr_type: "StatementClose".to_string(),
            opr_details: format!("Closing statement {}", stmt.financial_stmt.id),
            opr_src: Some("AccountStmtService".to_string()),
            pst_time: stmt.financial_stmt.pst_time,
            pst_type: PostingType::BalStmt,
            pst_status: PostingStatus::Posted,
            ledger: ledger_bo,
            val_time: Some(Utc::now()),
            lines: vec![],
            discarded_id: None,
            discarded_time: None,
            discarding_id: None,
            hash_record: Default::default(),
        };

        let antecedent = self
            .shared
            .posting_repo
            .find_first_by_ledger_order_by_record_time_desc(
                &closing_posting.ledger.named.id.to_string(),
            )
            .await
            .map_err(|_| ServiceError::Db)?;
        if let Some(ant) = antecedent {
            closing_posting.hash_record.antecedent_id = Some(Uuid::parse_str(&ant.id).unwrap());
            closing_posting.hash_record.antecedent_hash = ant.hash;
        }
        let hash = hash_serialize(&closing_posting).map_err(|_| ServiceError::NotEnoughInfo)?;
        closing_posting.hash_record.hash = Some(hash);

        let opr_details_id = self
            .shared
            .posting_repo
            .save_details(&closing_posting.opr_details)
            .await
            .map_err(|_| ServiceError::Db)?;
        let posting_model = PostingMapper::to_model(closing_posting.clone(), opr_details_id);
        self.shared
            .posting_repo
            .save(posting_model)
            .await
            .map_err(|_| ServiceError::Db)?;

        stmt_model.stmt_status = StmtStatus::Closed;
        stmt_model.posting_id = Some(closing_posting.id.to_string());
        self.shared
            .stmt_repo
            .save(stmt_model.clone())
            .await
            .map_err(|_| ServiceError::Db)?;

        let mut closed_stmt_bo = stmt;
        closed_stmt_bo.financial_stmt.stmt_status =
            postings_api::domain::stmt_status::StmtStatus::CLOSED;
        closed_stmt_bo.financial_stmt.posting = Some(closing_posting);

        Ok(closed_stmt_bo)
    }
}
