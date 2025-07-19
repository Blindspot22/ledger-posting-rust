use async_trait::async_trait;
use sqlx::{MySqlPool, query_as};
use uuid::Uuid;
use postings_db::models::named::{Named as DomainNamed, ContainerType as DomainContainerType};
use postings_db::repositories::named_repository::NamedRepository;
use postings_db::DbError;
use crate::models::named::{Named as MariaDbNamed, ContainerType as MariaDbContainerType};

pub struct MariaDbNamedRepository {
    pool: MySqlPool,
}

impl MariaDbNamedRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    fn to_domain(maria_named: MariaDbNamed) -> DomainNamed {
        DomainNamed {
            id: Uuid::parse_str(&maria_named.id).unwrap(),
            container: Uuid::parse_str(&maria_named.container).unwrap(),
            context: Uuid::parse_str(&maria_named.context).unwrap(),
            name: maria_named.name,
            language: maria_named.language,
            created: maria_named.created,
            user_details: maria_named.user_details.try_into().unwrap_or([0u8; 34]),
            short_desc: maria_named.short_desc,
            long_desc: maria_named.long_desc,
            container_type: match maria_named.container_type {
                MariaDbContainerType::ChartOfAccount => DomainContainerType::ChartOfAccount,
                MariaDbContainerType::Ledger => DomainContainerType::Ledger,
                MariaDbContainerType::LedgerAccount => DomainContainerType::LedgerAccount,
            }
        }
    }

    fn from_domain(domain_named: DomainNamed) -> MariaDbNamed {
        MariaDbNamed {
            id: domain_named.id.to_string(),
            container: domain_named.container.to_string(),
            context: domain_named.context.to_string(),
            name: domain_named.name,
            language: domain_named.language,
            created: domain_named.created,
            user_details: domain_named.user_details.to_vec(),
            short_desc: domain_named.short_desc,
            long_desc: domain_named.long_desc,
            container_type: match domain_named.container_type {
                DomainContainerType::ChartOfAccount => MariaDbContainerType::ChartOfAccount,
                DomainContainerType::Ledger => MariaDbContainerType::Ledger,
                DomainContainerType::LedgerAccount => MariaDbContainerType::LedgerAccount,
            }
        }
    }

    fn convert_container_type(domain_type: DomainContainerType) -> MariaDbContainerType {
        match domain_type {
            DomainContainerType::ChartOfAccount => MariaDbContainerType::ChartOfAccount,
            DomainContainerType::Ledger => MariaDbContainerType::Ledger,
            DomainContainerType::LedgerAccount => MariaDbContainerType::LedgerAccount,
        }
    }
}

#[async_trait]
impl NamedRepository for MariaDbNamedRepository {
    async fn find_by_container(&self, container_id: Uuid) -> Result<Vec<DomainNamed>, DbError> {
        let results: Vec<MariaDbNamed> = query_as::<_, MariaDbNamed>("SELECT * FROM named WHERE container = ?")
            .bind(container_id.to_string())
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DbError::from(e))?;
        
        Ok(results.into_iter().map(Self::to_domain).collect())
    }

    async fn find_by_name_and_type(&self, name: &str, container_type: DomainContainerType) -> Result<Vec<DomainNamed>, DbError> {
        let mariadb_type = Self::convert_container_type(container_type);
        let results: Vec<MariaDbNamed> = query_as::<_, MariaDbNamed>("SELECT * FROM named WHERE name = ? AND container_type = ?")
            .bind(name)
            .bind(mariadb_type)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DbError::from(e))?;
        
        Ok(results.into_iter().map(Self::to_domain).collect())
    }

    async fn find_by_name_and_type_and_context(&self, name: &str, container_type: DomainContainerType, context: Uuid) -> Result<Vec<DomainNamed>, DbError> {
        let mariadb_type = Self::convert_container_type(container_type);
        let results: Vec<MariaDbNamed> = query_as::<_, MariaDbNamed>("SELECT * FROM named WHERE name = ? AND container_type = ? AND context = ?")
            .bind(name)
            .bind(mariadb_type)
            .bind(context.to_string())
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DbError::from(e))?;
        
        Ok(results.into_iter().map(Self::to_domain).collect())
    }

    async fn save(&self, named: DomainNamed) -> Result<DomainNamed, DbError> {
        let maria_named = Self::from_domain(named);
        
        sqlx::query(
            "INSERT INTO named (id, container, context, name, language, created, user_details, short_desc, long_desc, container_type)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
             ON DUPLICATE KEY UPDATE
                container = VALUES(container),
                context = VALUES(context),
                name = VALUES(name),
                language = VALUES(language),
                created = VALUES(created),
                user_details = VALUES(user_details),
                short_desc = VALUES(short_desc),
                long_desc = VALUES(long_desc),
                container_type = VALUES(container_type)")
            .bind(&maria_named.id)
            .bind(&maria_named.container)
            .bind(&maria_named.context)
            .bind(&maria_named.name)
            .bind(&maria_named.language)
            .bind(maria_named.created)
            .bind(&maria_named.user_details)
            .bind(&maria_named.short_desc)
            .bind(&maria_named.long_desc)
            .bind(maria_named.container_type)
            .execute(&self.pool)
            .await
            .map_err(|e| DbError::from(e))?;
            
        // MariaDB does not support RETURNING, so we have to fetch it again
        let result: MariaDbNamed = query_as::<_, MariaDbNamed>("SELECT * FROM named WHERE id = ?")
            .bind(&maria_named.id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| DbError::from(e))?;
            
        Ok(Self::to_domain(result))
    }
}
