use std::sync::Arc;
use async_trait::async_trait;
use mockall::mock;
use postings_db::models::chart_of_account::ChartOfAccount;
use postings_db::repositories::chart_of_account_repository::ChartOfAccountRepository;
use postings_db::DbError;
use postings_logic::caching::chart_of_account_repository::CachingChartOfAccountRepository;
use uuid::Uuid;

mock! {
    pub ChartOfAccountRepository {}

    #[async_trait]
    impl ChartOfAccountRepository for ChartOfAccountRepository {
        async fn find_by_id(&self, id: Uuid) -> Result<Option<ChartOfAccount>, DbError> {
            unimplemented!()
        }
        async fn save(&self, coa: &ChartOfAccount) -> Result<(), DbError> {
            unimplemented!()
        }
    }
}

fn create_test_coa(id: Uuid) -> ChartOfAccount {
    ChartOfAccount {
        id,
    }
}

#[tokio::test]
async fn test_find_by_id_caches_result() {
    // Arrange
    let mut mock_repo = MockChartOfAccountRepository::new();
    let coa_id = Uuid::new_v4();
    let coa = create_test_coa(coa_id);
    
    let coa_clone = coa.clone();
    mock_repo.expect_find_by_id()
        .withf(move |id| *id == coa_id)
        .times(1) // Should only be called once
        .returning(move |_| Ok(Some(coa_clone.clone())));

    let caching_repo = CachingChartOfAccountRepository::new(Arc::new(mock_repo));

    // Act
    let result1 = caching_repo.find_by_id(coa_id).await.unwrap();
    let result2 = caching_repo.find_by_id(coa_id).await.unwrap();

    // Assert
    assert_eq!(result1, result2);
}

#[tokio::test]
async fn test_save_invalidates_caches() {
    // Arrange
    let mut mock_repo = MockChartOfAccountRepository::new();
    let coa_id = Uuid::new_v4();
    let coa = create_test_coa(coa_id);
    
    let coa_clone = coa.clone();
    let coa_clone2 = coa.clone();

    // Expect find_by_id to be called twice: once to populate the cache, and once after the cache is invalidated.
    mock_repo.expect_find_by_id()
        .withf(move |id| *id == coa_id)
        .times(2)
        .returning(move |_| Ok(Some(coa_clone.clone())));

    // Expect save to be called once.
    mock_repo.expect_save()
        .withf(move |c| c.id == coa_clone2.id)
        .times(1)
        .returning(move |_| Ok(()));

    let caching_repo = CachingChartOfAccountRepository::new(Arc::new(mock_repo));

    // Act & Assert
    // 1. Populate cache. This should call the mock repo's find_by_id.
    let _ = caching_repo.find_by_id(coa_id).await.unwrap();

    // 2. This call should hit the cache, so the mock repo's find_by_id should not be called again.
    let _ = caching_repo.find_by_id(coa_id).await.unwrap();

    // 3. Save, which should invalidate the cache.
    let _ = caching_repo.save(&coa).await.unwrap();

    // 4. Find again, should hit the mock repo again.
    let _ = caching_repo.find_by_id(coa_id).await.unwrap();
}
