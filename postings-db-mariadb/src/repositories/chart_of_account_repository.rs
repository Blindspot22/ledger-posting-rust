use uuid::Uuid;
use async_trait::async_trait;
use sqlx::MySqlPool;
use postings_db::repositories::chart_of_account_repository::ChartOfAccountRepository;
use postings_db::models::chart_of_account::ChartOfAccount as DomainChartOfAccount;
use crate::models::chart_of_account::ChartOfAccount as MariaDbChartOfAccount;
use postings_db::DbError;

pub struct MariaDbChartOfAccountRepository {
    pool: MySqlPool,
}

impl MariaDbChartOfAccountRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    fn to_domain(mariadb_coa: MariaDbChartOfAccount) -> DomainChartOfAccount {
        DomainChartOfAccount {
            id: Uuid::parse_str(&mariadb_coa.id).unwrap(),
        }
    }

    fn from_domain(domain_coa: &DomainChartOfAccount) -> MariaDbChartOfAccount {
        MariaDbChartOfAccount {
            id: domain_coa.id.to_string(),
        }
    }
}

#[async_trait]
impl ChartOfAccountRepository for MariaDbChartOfAccountRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainChartOfAccount>, DbError> {
        let result: Option<MariaDbChartOfAccount> = sqlx::query_as("SELECT * FROM chart_of_account WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(DbError::from)?;
        
        Ok(result.map(Self::to_domain))
    }

    async fn save(&self, coa: &DomainChartOfAccount) -> Result<(), DbError> {
        let mariadb_coa = Self::from_domain(coa);
        sqlx::query("INSERT INTO chart_of_account (id) VALUES (?)")
            .bind(mariadb_coa.id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
