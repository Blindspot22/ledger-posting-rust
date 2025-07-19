use async_trait::async_trait;
use crate::domain::named::Named;
use crate::ServiceError;
use uuid::Uuid;

#[async_trait]
pub trait NamedService {
    async fn find_by_container_id(&self, container_id: Uuid) -> Result<Vec<Named>, ServiceError>;
    async fn find_by_name_and_type(&self, name: String, container_type: ContainerType) -> Result<Vec<Named>, ServiceError>;
}
