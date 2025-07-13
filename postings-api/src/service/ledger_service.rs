use std::collections::HashMap;
use async_trait::async_trait;
use crate::domain::ledger::Ledger;
use crate::domain::ledger_account::LedgerAccount;
use crate::ServiceError;
use uuid::Uuid;

#[async_trait]
pub trait LedgerService {
    async fn new_ledger(&self, ledger: Ledger) -> Result<Ledger, ServiceError>;
    async fn find_ledger_by_id(&self, id: Uuid) -> Result<Option<Ledger>, ServiceError>;
    async fn find_ledger_by_name(&self, name: &str) -> Result<Option<Ledger>, ServiceError>;
    async fn new_ledger_account(&self, ledger_account: LedgerAccount, user_name: &str) -> Result<LedgerAccount, ServiceError>;
    async fn find_ledger_account_by_id(&self, id: Uuid) -> Result<LedgerAccount, ServiceError>;
    async fn find_ledger_account(&self, ledger: Ledger, name: &str) -> Result<LedgerAccount, ServiceError>;
    async fn check_if_ledger_account_exist(&self, ledger: Ledger, name: &str) -> Result<bool, ServiceError>;
    async fn find_ledger_accounts_by_ibans(&self, ibans: Vec<String>, ledger: Ledger) -> Result<HashMap<String, LedgerAccount>, ServiceError>;
}
