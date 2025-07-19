use uuid::Uuid;
use async_trait::async_trait;
use sqlx::PgPool;
use postings_db::repositories::chart_of_account_repository::ChartOfAccountRepository;
use postings_db::models::chart_of_account::ChartOfAccount;
use postings_db::DbError;

pub struct PostgresChartOfAccountRepository {
    pool: PgPool,
}

impl PostgresChartOfAccountRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ChartOfAccountRepository for PostgresChartOfAccountRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ChartOfAccount>, DbError> {
        sqlx::query_as("SELECT * FROM chart_of_account WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(DbError::from)
    }

    async fn save(&self, coa: &ChartOfAccount) -> Result<(), DbError> {
        sqlx::query("INSERT INTO chart_of_account (id) VALUES ($1)")
            .bind(coa.id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
