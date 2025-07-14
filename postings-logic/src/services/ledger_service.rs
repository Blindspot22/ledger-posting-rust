use async_trait::async_trait;
use postings_api::domain::ledger::Ledger;
use postings_api::domain::ledger_account::LedgerAccount;
use postings_api::service::ledger_service::LedgerService;
use postings_api::ServiceError;
use crate::services::shared_service::SharedService;
use std::collections::HashMap;
use uuid::Uuid;
use crate::mappers::ledger::LedgerMapper;
use crate::mappers::ledger_account::LedgerAccountMapper;
use postings_api::domain::account_category::AccountCategory as AccountCategoryBO;
use postings_api::domain::balance_side::BalanceSide as BalanceSideBO;

pub struct LedgerServiceImpl {
    shared: SharedService,
}

impl LedgerServiceImpl {
    pub fn new(shared: SharedService) -> Self {
        Self { shared }
    }
}

#[async_trait]
impl LedgerService for LedgerServiceImpl {
    async fn new_ledger(&self, mut ledger: Ledger) -> Result<Ledger, ServiceError> {
        ledger.named.id = Uuid::new_v4();
        ledger.named.created = chrono::Utc::now();
        let _coa = self.shared.load_coa(&ledger.coa).await?;
        let model = LedgerMapper::to_model(ledger);
        let saved_model = self.shared.ledger_repo.save(model).await.map_err(|_| ServiceError::Db)?;
        let coa_bo = self.shared.coa_repo.find_by_id(saved_model.coa_id).await.map_err(|_| ServiceError::Db)?.map(crate::mappers::chart_of_account::ChartOfAccountMapper::to_bo).ok_or(ServiceError::ChartOfAccountNotFound)?;
        Ok(LedgerMapper::to_bo(saved_model, coa_bo))
    }

    async fn find_ledger_by_id(&self, id: Uuid) -> Result<Option<Ledger>, ServiceError> {
        let ledger_model = self.shared.ledger_repo.find_by_id(id).await.map_err(|_| ServiceError::Db)?;
        if let Some(model) = ledger_model {
            let coa_bo = self.shared.coa_repo.find_by_id(model.coa_id).await.map_err(|_| ServiceError::Db)?.map(crate::mappers::chart_of_account::ChartOfAccountMapper::to_bo).ok_or(ServiceError::ChartOfAccountNotFound)?;
            Ok(Some(LedgerMapper::to_bo(model, coa_bo)))
        } else {
            Ok(None)
        }
    }

    async fn find_ledger_by_name(&self, name: &str) -> Result<Option<Ledger>, ServiceError> {
        let ledger_model = self.shared.ledger_repo.find_by_name(name).await.map_err(|_| ServiceError::Db)?;
        if let Some(model) = ledger_model {
            let coa_bo = self.shared.coa_repo.find_by_id(model.coa_id).await.map_err(|_| ServiceError::Db)?.map(crate::mappers::chart_of_account::ChartOfAccountMapper::to_bo).ok_or(ServiceError::ChartOfAccountNotFound)?;
            Ok(Some(LedgerMapper::to_bo(model, coa_bo)))
        } else {
            Ok(None)
        }
    }

    async fn new_ledger_account(&self, mut ledger_account: LedgerAccount, user_name: &str) -> Result<LedgerAccount, ServiceError> {
        ledger_account.named.id = Uuid::new_v4();
        ledger_account.named.created = chrono::Utc::now();
        ledger_account.named.user_details = user_name.to_string();

        let parent = if let Some(parent_bo) = ledger_account.parent.as_ref() {
            self.shared.load_ledger_account(parent_bo).await?
        } else {
            None
        };

        if ledger_account.category == AccountCategoryBO::NOOP {
            if let Some(p) = &parent {
                ledger_account.category = match p.category {
                    postings_db::models::account_category::AccountCategory::RE => AccountCategoryBO::RE,
                    postings_db::models::account_category::AccountCategory::EX => AccountCategoryBO::EX,
                    postings_db::models::account_category::AccountCategory::AS => AccountCategoryBO::AS,
                    postings_db::models::account_category::AccountCategory::LI => AccountCategoryBO::LI,
                    postings_db::models::account_category::AccountCategory::EQ => AccountCategoryBO::EQ,
                    postings_db::models::account_category::AccountCategory::NOOP => AccountCategoryBO::NOOP,
                    postings_db::models::account_category::AccountCategory::NORE => AccountCategoryBO::NORE,
                    postings_db::models::account_category::AccountCategory::NOEX => AccountCategoryBO::NOEX,
                };
            } else {
                return Err(ServiceError::NoCategory);
            }
        }
        
        if ledger_account.balance_side == BalanceSideBO::DrCr {
            if let Some(p) = &parent {
                ledger_account.balance_side = match p.balance_side {
                    postings_db::models::balance_side::BalanceSide::Dr => BalanceSideBO::Dr,
                    postings_db::models::balance_side::BalanceSide::Cr => BalanceSideBO::Cr,
                    postings_db::models::balance_side::BalanceSide::DrCr => BalanceSideBO::DrCr,
                };
            } else {
                ledger_account.balance_side = ledger_account.category.default_bs();
            }
        }

        let model = LedgerAccountMapper::to_model(ledger_account);
        let saved_model = self.shared.ledger_account_repo.save(model).await.map_err(|_| ServiceError::Db)?;
        
        let ledger_bo = self.find_ledger_by_id(saved_model.ledger_id).await?.ok_or(ServiceError::LedgerNotFound)?;
        let coa_bo = self.shared.coa_repo.find_by_id(saved_model.coa_id).await.map_err(|_| ServiceError::Db)?.map(crate::mappers::chart_of_account::ChartOfAccountMapper::to_bo).ok_or(ServiceError::ChartOfAccountNotFound)?;
        let parent_bo = if let Some(parent_id) = saved_model.parent_id {
            Some(Box::new(self.find_ledger_account_by_id(parent_id).await?))
        } else {
            None
        };

        Ok(LedgerAccountMapper::to_bo(saved_model, ledger_bo, coa_bo, parent_bo))
    }

    async fn find_ledger_account_by_id(&self, id: Uuid) -> Result<LedgerAccount, ServiceError> {
        let model = self.shared.ledger_account_repo
            .find_by_id(id)
            .await
            .map_err(|_| ServiceError::Db)?;
            
        let ledger_bo = self.find_ledger_by_id(model.ledger_id).await?.ok_or(ServiceError::LedgerNotFound)?;
        let coa_bo = self.shared.coa_repo
            .find_by_id(model.coa_id)
            .await
            .map_err(|_| ServiceError::Db)?
            .map(crate::mappers::chart_of_account::ChartOfAccountMapper::to_bo)
            .ok_or(ServiceError::ChartOfAccountNotFound)?;
        let parent_bo = if let Some(parent_id) = model.parent_id {
            Some(Box::new(self.find_ledger_account_by_id(parent_id).await?))
        } else {
            None
        };
        Ok(LedgerAccountMapper::to_bo(model, ledger_bo, coa_bo, parent_bo))
    }

    async fn find_ledger_account(&self, ledger: Ledger, name: &str) -> Result<LedgerAccount, ServiceError> {
        let model = self.shared.ledger_account_repo
            .find_by_ledger_and_name(&ledger.named.id.to_string(), name)
            .await
            .map_err(|_| ServiceError::Db)?
            .ok_or(ServiceError::LedgerAccountNotFound)?;
            
        let ledger_bo = self.find_ledger_by_id(model.ledger_id).await?.ok_or(ServiceError::LedgerNotFound)?;
        let coa_bo = self.shared.coa_repo.find_by_id(model.coa_id).await.map_err(|_| ServiceError::Db)?.map(crate::mappers::chart_of_account::ChartOfAccountMapper::to_bo).ok_or(ServiceError::ChartOfAccountNotFound)?;
        let parent_bo = if let Some(parent_id) = model.parent_id {
            Some(Box::new(self.find_ledger_account_by_id(parent_id).await?))
        } else {
            None
        };
        Ok(LedgerAccountMapper::to_bo(model, ledger_bo, coa_bo, parent_bo))
    }

    async fn check_if_ledger_account_exist(&self, ledger: Ledger, name: &str) -> Result<bool, ServiceError> {
        let result = self.shared.ledger_account_repo.find_by_ledger_and_name(&ledger.named.id.to_string(), name).await.map_err(|_| ServiceError::Db)?;
        Ok(result.is_some())
    }

    async fn find_ledger_accounts_by_ibans(&self, ibans: Vec<String>, ledger: Ledger) -> Result<HashMap<String, LedgerAccount>, ServiceError> {
        let models = self.shared.ledger_account_repo
            .find_by_ibans(ibans.into_iter().collect(), &ledger.named.id.to_string())
            .await
            .map_err(|_| ServiceError::Db)?;
            
        let mut result = HashMap::new();
        for model in models {
            let ledger_bo = self.find_ledger_by_id(model.ledger_id).await?.ok_or(ServiceError::LedgerNotFound)?;
            let coa_bo = self.shared.coa_repo.find_by_id(model.coa_id).await.map_err(|_| ServiceError::Db)?.map(crate::mappers::chart_of_account::ChartOfAccountMapper::to_bo).ok_or(ServiceError::ChartOfAccountNotFound)?;
            let parent_bo = if let Some(parent_id) = model.parent_id {
                Some(Box::new(self.find_ledger_account_by_id(parent_id).await?))
            } else {
                None
            };
            let bo = LedgerAccountMapper::to_bo(model, ledger_bo, coa_bo, parent_bo);
            result.insert(bo.named.name.clone(), bo);
        }
        Ok(result)
    }
}