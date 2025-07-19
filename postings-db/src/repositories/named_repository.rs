use async_trait::async_trait;
use crate::models::named::{Named, ContainerType};

use crate::DbError;
use uuid::Uuid;

#[async_trait]
pub trait NamedRepository {
    async fn find_by_container(&self, container_id: Uuid) -> Result<Vec<Named>, DbError>;
    async fn find_by_name_and_type(&self, name: &str, container_type: ContainerType) -> Result<Vec<Named>, DbError>;
    async fn find_by_name_and_type_and_context(&self, name: &str, container_type: ContainerType, context: Uuid) -> Result<Vec<Named>, DbError>;
    async fn save(&self, named: Named) -> Result<Named, DbError>;
}