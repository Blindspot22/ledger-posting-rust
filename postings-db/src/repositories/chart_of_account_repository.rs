use async_trait::async_trait;
use crate::models::chart_of_account::ChartOfAccount;
use crate::DbError;

#[async_trait]
pub trait ChartOfAccountRepository {
    async fn find_by_name(&self, name: &str) -> Result<Option<ChartOfAccount>, DbError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<ChartOfAccount>, DbError>;
    async fn save(&self, coa: ChartOfAccount) -> Result<ChartOfAccount, DbError>;
}
