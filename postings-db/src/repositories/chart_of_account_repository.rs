use async_trait::async_trait;
use crate::models::chart_of_account::ChartOfAccount;
use crate::DbError;
use uuid::Uuid;

#[async_trait]
pub trait ChartOfAccountRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ChartOfAccount>, DbError>;
    async fn save(&self, coa: &ChartOfAccount) -> Result<(), DbError>;
}
