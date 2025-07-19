use crate::mappers::named::NamedMapper;
use crate::services::shared_service::SharedService;
use async_trait::async_trait;
use postings_api::domain::named::{Named, ContainerType};
use postings_api::service::named_service::NamedService;
use postings_api::ServiceError;
use uuid::Uuid;

pub struct NamedServiceImpl {
    shared: SharedService,
}

impl NamedServiceImpl {
    pub fn new(shared: SharedService) -> Self {
        Self { shared }
    }
}

#[async_trait]
impl NamedService for NamedServiceImpl {
    async fn find_by_container_id(&self, container_id: Uuid) -> Result<Vec<Named>, ServiceError> {
        let named_models = self
            .shared
            .named_repo
            .find_by_container(container_id)
            .await
            .map_err(|_| ServiceError::Db)?;
        Ok(named_models.into_iter().map(NamedMapper::to_bo).collect())
    }
    async fn find_by_name_and_type(&self, name: String, container_type: ContainerType) -> Result<Vec<Named>, ServiceError> {
        let named_models = self
            .shared
            .named_repo
            .find_by_name_and_type(name, container_type)
            .await
            .map_err(|_| ServiceError::Db)?;
        Ok(named_models.into_iter().map(NamedMapper::to_bo).collect())
    }
}
