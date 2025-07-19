use std::sync::Arc;
use async_trait::async_trait;
use moka::future::Cache;
use postings_db::models::chart_of_account::ChartOfAccount;
use postings_db::repositories::chart_of_account_repository::ChartOfAccountRepository;
use postings_db::DbError;
use uuid::Uuid;

pub struct CachingChartOfAccountRepository {
    inner: Arc<dyn ChartOfAccountRepository + Send + Sync>,
    cache_by_id: Cache<Uuid, ChartOfAccount>,
}

impl CachingChartOfAccountRepository {
    pub fn new(inner: Arc<dyn ChartOfAccountRepository + Send + Sync>) -> Self {
        Self {
            inner,
            cache_by_id: Cache::new(1000),
        }
    }
}

#[async_trait]
impl ChartOfAccountRepository for CachingChartOfAccountRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ChartOfAccount>, DbError> {
        if let Some(coa) = self.cache_by_id.get(&id).await {
            return Ok(Some(coa));
        }

        let coa_opt = self.inner.find_by_id(id).await?;
        if let Some(coa) = &coa_opt {
            self.cache_by_id.insert(id, coa.clone()).await;
        }
        Ok(coa_opt)
    }

    async fn save(&self, coa: &ChartOfAccount) -> Result<(), DbError> {
        self.inner.save(coa).await?;
        self.cache_by_id.invalidate(&coa.id).await;
        Ok(())
    }
}
