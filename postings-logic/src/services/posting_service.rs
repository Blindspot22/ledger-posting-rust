use async_trait::async_trait;
use postings_api::domain::posting::Posting;
use postings_api::domain::posting_line::PostingLine;
use postings_api::domain::ledger_account::LedgerAccount;
use postings_api::service::posting_service::{PostingService, Page};
use postings_api::ServiceError;
use crate::services::shared_service::SharedService;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use bigdecimal::BigDecimal;
use crate::hash_utils::hash_serialize;
use crate::mappers::posting::PostingMapper;

pub struct PostingServiceImpl {
    shared: SharedService,
    // posting_repo, stmt_repo, line_repo would be here
}

impl PostingServiceImpl {
    pub fn new(shared: SharedService) -> Self {
        Self { shared }
    }
}

#[async_trait]
impl PostingService for PostingServiceImpl {
    async fn new_posting(&self, mut posting: Posting) -> Result<Posting, ServiceError> {
        let debit_sum: BigDecimal = posting.lines.iter().map(|l| l.debit_amount.clone()).sum();
        let credit_sum: BigDecimal = posting.lines.iter().map(|l| l.credit_amount.clone()).sum();

        if debit_sum != credit_sum {
            return Err(ServiceError::DoubleEntry);
        }

        posting.id = Uuid::new_v4();
        posting.record_time = Utc::now();

        // Simplified predecessor logic
        if let Some(_predecessor) = self.shared.posting_repo.find_by_opr_id_and_discarding_id_is_null(&posting.opr_id).await.map_err(|_| ServiceError::Db)? {
            // Discard predecessor, not fully implemented here
        }

        let antecedent = self.shared.posting_repo.find_first_by_ledger_order_by_record_time_desc(&posting.ledger.named.id.to_string()).await.map_err(|_| ServiceError::Db)?;
        if let Some(ant) = antecedent {
            posting.hash_record.antecedent_id = Some(Uuid::parse_str(&ant.id).unwrap());
            posting.hash_record.antecedent_hash = ant.hash;
        }
        
        let hash = hash_serialize(&posting).map_err(|_| ServiceError::NotEnoughInfo)?; // Simplified error
        posting.hash_record.hash = Some(hash);

        let opr_details_id = self.shared.posting_repo.save_details(&posting.opr_details).await.map_err(|_| ServiceError::Db)?;
        let db_posting = PostingMapper::to_model(posting.clone(), opr_details_id);
        self.shared.posting_repo.save(db_posting).await.map_err(|_| ServiceError::Db)?;
        
        Ok(posting)
    }

    async fn find_postings_by_operation_id(&self, opr_id: &str) -> Result<Vec<Posting>, ServiceError> {
        // Simplified, mapping needed
        self.shared.posting_repo.find_by_opr_id(opr_id).await.map_err(|_| ServiceError::Db)?;
        Ok(vec![])
    }

    async fn find_postings_by_dates(&self, ledger_account: LedgerAccount, date_from: DateTime<Utc>, date_to: DateTime<Utc>) -> Result<Vec<PostingLine>, ServiceError> {
        // Simplified, mapping needed
        self.shared.line_repo.find_by_account_and_pst_time_between(&ledger_account.named.id.to_string(), date_from, date_to).await.map_err(|_| ServiceError::Db)?;
        Ok(vec![])
    }

    async fn find_postings_by_dates_paged(&self, ledger_account: LedgerAccount, date_from: DateTime<Utc>, date_to: DateTime<Utc>, _page: usize, _size: usize) -> Result<Page<PostingLine>, ServiceError> {
        // Simplified, proper pagination and mapping needed
        let lines = self.shared.line_repo.find_by_account_and_pst_time_between(&ledger_account.named.id.to_string(), date_from, date_to).await.map_err(|_| ServiceError::Db)?;
        Ok(Page { content: vec![], total_elements: lines.len() as u64 })
    }

    async fn find_posting_line_by_id(&self, ledger_account: LedgerAccount, transaction_id: Uuid) -> Result<PostingLine, ServiceError> {
        // Simplified, mapping needed
        self.shared.line_repo.find_by_id_and_account_id(&transaction_id.to_string(), &ledger_account.named.id.to_string()).await.map_err(|_| ServiceError::Db)?;
        Err(ServiceError::PostingNotFound)
    }
}