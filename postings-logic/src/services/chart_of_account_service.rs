use async_trait::async_trait;
use postings_api::domain::chart_of_account::ChartOfAccount;
use postings_api::service::chart_of_account_service::ChartOfAccountService;
use postings_api::ServiceError;
use crate::services::shared_service::SharedService;
use uuid::Uuid;
use crate::mappers::chart_of_account::ChartOfAccountMapper;

pub struct ChartOfAccountServiceImpl {
    shared: SharedService,
}

impl ChartOfAccountServiceImpl {
    pub fn new(shared: SharedService) -> Self {
        Self { shared }
    }
}

#[async_trait]
impl ChartOfAccountService for ChartOfAccountServiceImpl {
    async fn new_chart_of_account(&self, mut chart_of_account: ChartOfAccount) -> Result<ChartOfAccount, ServiceError> {
        chart_of_account.named.id = Uuid::new_v4();
        chart_of_account.named.created = chrono::Utc::now();
        let model = ChartOfAccountMapper::to_model(chart_of_account);
        let saved_model = self.shared.coa_repo.save(model).await.map_err(|_| ServiceError::Db)?;
        Ok(ChartOfAccountMapper::to_bo(saved_model))
    }

    async fn find_chart_of_accounts_by_name(&self, name: &str) -> Result<Option<ChartOfAccount>, ServiceError> {
        let coa_model = self.shared.coa_repo.find_by_name(name).await.map_err(|_| ServiceError::Db)?;
        Ok(coa_model.map(ChartOfAccountMapper::to_bo))
    }

    async fn find_chart_of_accounts_by_id(&self, id: Uuid) -> Result<Option<ChartOfAccount>, ServiceError> {
        let coa_model = self.shared.coa_repo.find_by_id(&id.to_string()).await.map_err(|_| ServiceError::Db)?;
        Ok(coa_model.map(ChartOfAccountMapper::to_bo))
    }
}