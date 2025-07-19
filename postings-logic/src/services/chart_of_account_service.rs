use async_trait::async_trait;
use postings_api::domain::chart_of_account::ChartOfAccount;
use postings_api::service::chart_of_account_service::ChartOfAccountService;
use postings_api::ServiceError;
use postings_db::models::named::ContainerType;
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

//use postings_api::domain::named::{ContainerType, Named};
use crate::mappers::named::NamedMapper;

#[async_trait]
impl ChartOfAccountService for ChartOfAccountServiceImpl {
    async fn new_chart_of_account(&self, mut chart_of_account: ChartOfAccount, named: Vec<postings_api::domain::named::Named>) -> Result<(ChartOfAccount, Vec<postings_api::domain::named::Named>), ServiceError> {
        chart_of_account.id = Uuid::new_v4();
        let model = ChartOfAccountMapper::to_model(chart_of_account);
        self.shared.coa_repo.save(&model).await.map_err(|_| ServiceError::Db)?;
        let coa_bo = ChartOfAccountMapper::to_bo(model);

        let mut saved_named = Vec::new();
        for mut n in named {
            n.container = coa_bo.id;
            let named_model = NamedMapper::to_model(n);
            let saved_named_model = self.shared.named_repo.save(named_model).await.map_err(|_| ServiceError::Db)?;
            saved_named.push(NamedMapper::to_bo(saved_named_model));
        }

        Ok((coa_bo, saved_named))
    }

    async fn find_chart_of_accounts_by_name(&self, name: &str) -> Result<Vec<ChartOfAccount>, ServiceError> {
        let named_models = self.shared.named_repo
            .find_by_name_and_type(name, ContainerType::ChartOfAccount)
            .await
            .map_err(|_| ServiceError::Db)?;
        
        let mut chart_of_accounts = Vec::new();
        
        for nm in named_models {
            let coa_model = self.shared.coa_repo
                .find_by_id(nm.container)
                .await
                .map_err(|_| ServiceError::Db)?;
            
            if let Some(model) = coa_model {
                chart_of_accounts.push(ChartOfAccountMapper::to_bo(model));
            }
        }
        
        Ok(chart_of_accounts)
    }

    async fn find_chart_of_accounts_by_id(&self, id: Uuid) -> Result<Option<ChartOfAccount>, ServiceError> {
        let coa_model = self.shared.coa_repo.find_by_id(id).await.map_err(|_| ServiceError::Db)?;
        if let Some(cm) = coa_model {
            let coa_bo = ChartOfAccountMapper::to_bo(cm);
            Ok(Some(coa_bo))
        } else {
            Ok(None)
        }
    }
}