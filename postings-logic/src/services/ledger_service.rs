use crate::mappers::ledger::LedgerMapper;
use crate::mappers::ledger_account::LedgerAccountMapper;
use crate::services::chart_of_account_service::ChartOfAccountServiceImpl;
use crate::services::shared_service::SharedService;
use async_trait::async_trait;
use postings_api::domain::chart_of_account::ChartOfAccount;
use postings_api::domain::ledger::Ledger;
use postings_api::domain::ledger_account::LedgerAccount;
use postings_api::service::chart_of_account_service::ChartOfAccountService;
use postings_api::service::ledger_service::LedgerService;
use postings_api::ServiceError;
use postings_db::models::named::ContainerType;
use std::collections::HashMap;
use uuid::Uuid;

pub struct LedgerServiceImpl {
    shared: SharedService,
    coa_service: ChartOfAccountServiceImpl,
}

impl LedgerServiceImpl {
    pub fn new(shared: SharedService, coa_service: ChartOfAccountServiceImpl) -> Self {
        Self {
            shared,
            coa_service,
        }
    }

    async fn load_chart_of_account(&self, coa_id: Uuid) -> Result<ChartOfAccount, ServiceError> {
        self.coa_service
            .find_chart_of_accounts_by_id(coa_id)
            .await?
            .ok_or(ServiceError::ChartOfAccountNotFound)
    }

    async fn load_ledger_account_dependencies(&self, model: &postings_db::models::ledger_account::LedgerAccount) -> Result<(Ledger, ChartOfAccount, Option<Box<LedgerAccount>>), ServiceError> {
        let ledger_bo = self
            .find_ledger_by_id(model.ledger_id)
            .await?
            .ok_or(ServiceError::LedgerNotFound)?;
        let coa_bo = self.load_chart_of_account(model.coa_id).await?;
        let parent_bo = if let Some(parent_id) = model.parent_id {
            Some(Box::new(self.find_ledger_account_by_id(parent_id).await?.ok_or(ServiceError::LedgerAccountNotFound)?))
        } else {
            None
        };

        Ok((ledger_bo, coa_bo, parent_bo))
    }
}

use crate::mappers::named::NamedMapper;
use postings_api::domain::named::Named;

#[async_trait]
impl LedgerService for LedgerServiceImpl {

    async fn new_ledger(&self, ledger: Ledger, named: Vec<Named>) -> Result<(Ledger, Vec<Named>), ServiceError> {
        // Ensure the COA exists before saving the ledger
        // Retung an error if the COA is not found
        let coa_bo = self.load_chart_of_account(ledger.coa.id).await?;

        let model = LedgerMapper::to_model(ledger);
        self.shared
            .ledger_repo
            .save(&model)
            .await
            .map_err(|_| ServiceError::Db)?;

        let mut saved_named = Vec::new();
        for mut n in named {
            n.container = model.id;
            n.context = model.coa_id; // Ledger's context is its Chart of Account
            let named_model = NamedMapper::to_model(n);
            let saved_named_model = self
                .shared
                .named_repo
                .save(named_model)
                .await
                .map_err(|_| ServiceError::Db)?;
            saved_named.push(NamedMapper::to_bo(saved_named_model));
        }

        let ledger_bo = LedgerMapper::to_bo(model, coa_bo);

        Ok((ledger_bo, saved_named))
    }

    async fn find_ledger_by_id(&self, id: Uuid) -> Result<Option<Ledger>, ServiceError> {
        let ledger_model = self
            .shared
            .ledger_repo
            .find_by_id(id)
            .await
            .map_err(|_| ServiceError::Db)?;
        if let Some(model) = ledger_model {
            let coa_bo = self.load_chart_of_account(model.coa_id).await?;
            let ledger_bo = LedgerMapper::to_bo(model, coa_bo);
            Ok(Some(ledger_bo))
        } else {
            Ok(None)
        }
    }

    async fn find_ledger_by_name(&self, name: &str, coa_id: Uuid) -> Result<Vec<Ledger>, ServiceError> {
        // Then get the chart of accounts
        let coa_bo = self.load_chart_of_account(coa_id).await?;

        let named_models = self.shared
            .named_repo
            .find_by_name_and_type_and_context(name, ContainerType::Ledger, coa_id)
            .await
            .map_err(|_| ServiceError::Db)?;
        
        let mut ledgers = Vec::new();        
        for nm in named_models {
            // First get the ledger model
            let ledger_model = match self.shared
                .ledger_repo
                .find_by_id(nm.container)
                .await
                .map_err(|_| ServiceError::Db)? 
            {
                Some(model) => model,
                None => continue, // Skip if ledger not found
            };
            
            
            ledgers.push(LedgerMapper::to_bo(ledger_model, coa_bo.clone()));
        }
        
        Ok(ledgers)
    }

    async fn new_ledger_account(
        &self,
        ledger_account: LedgerAccount,
        named: Vec<Named>,
    ) -> Result<(LedgerAccount, Vec<Named>), ServiceError> {
        // Load the ledger to ensure it exists
        let leddger = self
            .shared
            .ledger_repo
            .find_by_id(ledger_account.ledger.id)
            .await
            .map_err(|_| ServiceError::Db)?
            .ok_or(ServiceError::LedgerNotFound)?;
        // Make sure the COA in the ledger is the same as the one referenced
        if leddger.coa_id != ledger_account.coa.id {
            return Err(ServiceError::ChartOfAccountMismatch);
        }

        let model = LedgerAccountMapper::to_model(ledger_account);
        self.shared
            .ledger_account_repo
            .save(&model)
            .await
            .map_err(|_| ServiceError::Db)?;

        let (ledger_bo, coa_bo, parent_bo) = self.load_ledger_account_dependencies(&model).await?;
        let la_bo = LedgerAccountMapper::to_bo(model, ledger_bo, coa_bo, parent_bo);

        let mut saved_named = Vec::new();
        for mut n in named {
            n.container = la_bo.id;
            n.context = la_bo.ledger.id; // LedgerAccount's context is its Ledger
            let named_model = NamedMapper::to_model(n);
            let saved_named_model = self
                .shared
                .named_repo
                .save(named_model)
                .await
                .map_err(|_| ServiceError::Db)?;
            saved_named.push(NamedMapper::to_bo(saved_named_model));
        }

        Ok((la_bo, saved_named))
    }

    async fn find_ledger_account_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<LedgerAccount>, ServiceError> {
        if let Some(model) = self
            .shared
            .ledger_account_repo
            .find_by_id(id)
            .await
            .map_err(|_| ServiceError::Db)?
        {
            let (ledger_bo, coa_bo, parent_bo) = self.load_ledger_account_dependencies(&model).await?;
            let la_bo = LedgerAccountMapper::to_bo(model, ledger_bo, coa_bo, parent_bo);
            Ok(Some(la_bo))
        } else {
            Ok(None)
        }
    }

    async fn find_ledger_account_by_name(
        &self,
        ledger: &Ledger,
        name: &str,
    ) -> Result<Vec<LedgerAccount>, ServiceError> {
        let nm = self
            .shared
            .named_repo
            .find_by_name_and_type(name, ContainerType::LedgerAccount)
            .await
            .map_err(|_| ServiceError::Db)?;
        let mut result = Vec::new();
        for named in nm {
            // load ledger account by named container
            if let Some(la_bo) = self.find_ledger_account_by_id(named.container).await? {
                // Only include if it belongs to the specified ledger
                if la_bo.ledger.id == ledger.id {
                    result.push(la_bo);
                }
            }
        }
        Ok(result)
    }

    async fn check_if_ledger_account_exist(
        &self,
        ledger: &Ledger,
        name: &str,
    ) -> Result<bool, ServiceError> {
        let result = self.find_ledger_account_by_name(ledger, name).await?;
        Ok(!result.is_empty())
    }

    async fn find_ledger_accounts_by_ibans(
        &self,
        ibans: Vec<String>,
        ledger: &Ledger,
    ) -> Result<HashMap<String, Vec<LedgerAccount>>, ServiceError> {
        let mut result = HashMap::new();
        for iban in ibans {
            let accounts = self.find_ledger_account_by_name(ledger, &iban).await?;
            if !accounts.is_empty() {
                result.insert(iban, accounts);
            }
        }
        Ok(result)
    }
}
