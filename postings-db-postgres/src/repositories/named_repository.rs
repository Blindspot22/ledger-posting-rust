use async_trait::async_trait;
use sqlx::{PgPool, query_as};
use uuid::Uuid;
use postings_db::models::named::{Named, ContainerType};
use postings_db::repositories::named_repository::NamedRepository;
use postings_db::DbError;

pub struct PostgresNamedRepository {
    pool: PgPool,
}

impl PostgresNamedRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NamedRepository for PostgresNamedRepository {
    async fn find_by_container(&self, container_id: Uuid) -> Result<Vec<Named>, DbError> {
        query_as::<_, Named>("SELECT * FROM named WHERE container = $1")
            .bind(container_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.into())
    }

    async fn find_by_name_and_type(&self, name: &str, container_type: ContainerType) -> Result<Vec<Named>, DbError> {
        query_as::<_, Named>("SELECT * FROM named WHERE name = $1 AND container_type = $2")
            .bind(name)
            .bind(container_type)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.into())
    }

    async fn find_by_name_and_type_and_context(&self, name: &str, container_type: ContainerType, context: Uuid) -> Result<Vec<Named>, DbError> {
        query_as::<_, Named>("SELECT * FROM named WHERE name = $1 AND container_type = $2 AND context = $3")
            .bind(name)
            .bind(container_type)
            .bind(context)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.into())
    }

    async fn save(&self, named: Named) -> Result<Named, DbError> {
        query_as::<_, Named>(
            "INSERT INTO named (id, container, context, name, language, created, user_details, short_desc, long_desc, container_type)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
             ON CONFLICT (id) DO UPDATE SET
                container = $2, context = $3, name = $4, language = $5, created = $6, user_details = $7, short_desc = $8, long_desc = $9, container_type = $10
             RETURNING *")
            .bind(named.id)
            .bind(named.container)
            .bind(named.context)
            .bind(named.name)
            .bind(named.language)
            .bind(named.created)
            .bind(named.user_details)
            .bind(named.short_desc)
            .bind(named.long_desc)
            .bind(named.container_type)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.into())
    }
}
