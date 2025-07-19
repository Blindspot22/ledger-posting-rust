use uuid::Uuid;
use async_trait::async_trait;
use sqlx::MySqlPool;
use postings_db::repositories::chart_of_account_repository::ChartOfAccountRepository;
use postings_db::models::chart_of_account::ChartOfAccount;
use postings_db::DbError;

pub struct MariaDbChartOfAccountRepository {
    pool: MySqlPool,
}

impl MariaDbChartOfAccountRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ChartOfAccountRepository for MariaDbChartOfAccountRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ChartOfAccount>, DbError> {
        sqlx::query_as("SELECT * FROM chart_of_account WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(DbError::from)
    }

    async fn save(&self, coa: &ChartOfAccount) -> Result<(), DbError> {
        sqlx::query("INSERT INTO chart_of_account (id) VALUES (?)")
            .bind(coa.id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
