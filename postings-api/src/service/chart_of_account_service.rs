use async_trait::async_trait;
use crate::domain::chart_of_account::ChartOfAccount;
use crate::domain::named::Named;
use crate::ServiceError;
use uuid::Uuid;

#[async_trait]
pub trait ChartOfAccountService {
    async fn new_chart_of_account(&self, chart_of_account: ChartOfAccount, named: Vec<Named>) -> Result<(ChartOfAccount, Vec<Named>), ServiceError>;
    async fn find_chart_of_accounts_by_name(&self, name: &str) -> Result<Vec<ChartOfAccount>, ServiceError>;
    async fn find_chart_of_accounts_by_id(&self, id: Uuid) -> Result<Option<ChartOfAccount>, ServiceError>;
}
