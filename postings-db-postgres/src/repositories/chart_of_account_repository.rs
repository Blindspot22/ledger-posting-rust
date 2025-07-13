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
    async fn find_by_name(&self, name: &str) -> Result<Option<ChartOfAccount>, DbError> {
        sqlx::query_as("SELECT * FROM chart_of_account WHERE name = $1")
            .bind(name)
            .fetch_optional(&self.pool)
            .await
            .map_err(DbError::from)
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<ChartOfAccount>, DbError> {
        sqlx::query_as("SELECT * FROM chart_of_account WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(DbError::from)
    }

    async fn save(&self, coa: ChartOfAccount) -> Result<ChartOfAccount, DbError> {
        sqlx::query_as("INSERT INTO chart_of_account (id, name, created, user_details, short_desc, long_desc) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *")
            .bind(coa.id)
            .bind(coa.name)
            .bind(coa.created)
            .bind(coa.user_details)
            .bind(coa.short_desc)
            .bind(coa.long_desc)
            .fetch_one(&self.pool)
            .await
            .map_err(DbError::from)
    }
}
