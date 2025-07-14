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
    async fn find_by_name(&self, name: &str) -> Result<Option<ChartOfAccount>, DbError> {
        sqlx::query_as("SELECT * FROM chart_of_account WHERE name = ?")
            .bind(name)
            .fetch_optional(&self.pool)
            .await
            .map_err(DbError::from)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<ChartOfAccount>, DbError> {
        sqlx::query_as("SELECT * FROM chart_of_account WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(DbError::from)
    }

    async fn save(&self, coa: ChartOfAccount) -> Result<ChartOfAccount, DbError> {
        sqlx::query("INSERT INTO chart_of_account (id, name, created, user_details, short_desc, long_desc) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(&coa.id)
            .bind(&coa.name)
            .bind(coa.created)
            .bind(&coa.user_details)
            .bind(&coa.short_desc)
            .bind(&coa.long_desc)
            .execute(&self.pool)
            .await?;
        Ok(coa)
    }
}
