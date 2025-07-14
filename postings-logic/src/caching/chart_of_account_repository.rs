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
    cache_by_name: Cache<String, ChartOfAccount>,
}

impl CachingChartOfAccountRepository {
    pub fn new(inner: Arc<dyn ChartOfAccountRepository + Send + Sync>) -> Self {
        Self {
            inner,
            cache_by_id: Cache::new(1000),
            cache_by_name: Cache::new(1000),
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
            self.cache_by_name.insert(coa.name.clone(), coa.clone()).await;
        }
        Ok(coa_opt)
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<ChartOfAccount>, DbError> {
        if let Some(coa) = self.cache_by_name.get(name).await {
            return Ok(Some(coa));
        }

        let coa_opt = self.inner.find_by_name(name).await?;
        if let Some(coa) = &coa_opt {
            self.cache_by_id.insert(coa.id, coa.clone()).await;
            self.cache_by_name.insert(name.to_string(), coa.clone()).await;
        }
        Ok(coa_opt)
    }

    async fn save(&self, coa: ChartOfAccount) -> Result<ChartOfAccount, DbError> {
        let saved_coa = self.inner.save(coa).await?;
        self.cache_by_id.invalidate(&saved_coa.id).await;
        self.cache_by_name.invalidate(&saved_coa.name).await;
        Ok(saved_coa)
    }
}
