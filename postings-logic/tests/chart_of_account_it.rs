#![cfg(test)]

#[cfg(feature = "postgres_tests")]
mod postgres_tests {
    use std::sync::Arc;
    use serde::Deserialize;
    use sqlx::{PgPool, Executor};
    use uuid::Uuid;
    use hex;
    use postings_logic::services::chart_of_account_service::ChartOfAccountServiceImpl;
    use postings_api::service::chart_of_account_service::ChartOfAccountService;
    use postings_db_postgres::repositories::chart_of_account_repository::PostgresChartOfAccountRepository;
    use postings_db_postgres::repositories::ledger_repository::PostgresLedgerRepository;
    use postings_db_postgres::repositories::ledger_account_repository::PostgresLedgerAccountRepository;
    use postings_db_postgres::repositories::posting_repository::PostgresPostingRepository;
    use postings_db_postgres::repositories::account_stmt_repository::PostgresAccountStmtRepository;
    use postings_db_postgres::repositories::posting_line_repository::PostgresPostingLineRepository;
    use postings_db_postgres::repositories::posting_trace_repository::PostgresPostingTraceRepository;
    use postings_db_postgres::repositories::named_repository::PostgresNamedRepository;
    use postings_logic::services::shared_service::SharedService;

    #[derive(Deserialize)]
    struct ChartOfAccountSeed {
        id: Uuid,
    }

    #[derive(Deserialize)]
    struct NamedSeed {
        id: Uuid,
        container: Uuid,
        context: Option<Uuid>,
        name: String,
        language: String,
        created: chrono::NaiveDateTime,
        user_details: String,
        short_desc: Option<String>,
        long_desc: Option<String>,
        container_type: Option<String>,
    }

    #[derive(Deserialize)]
    struct TestDataSet {
        charts_of_account: Vec<ChartOfAccountSeed>,
        named: Vec<NamedSeed>,
    }

    async fn setup_data(pool: &PgPool, file_path: &str) -> anyhow::Result<()> {
        let yaml_str = std::fs::read_to_string(file_path)?;
        let data_set: TestDataSet = serde_yaml::from_str(&yaml_str)?;

        // Insert chart of accounts (just ID)
        for coa in data_set.charts_of_account {
            pool.execute(sqlx::query("INSERT INTO chart_of_account (id) VALUES ($1)")
                .bind(coa.id)
            ).await?;
        }

        // Insert named entities
        for named in data_set.named {
            let container_type = named.container_type.unwrap_or("ChartOfAccount".to_string());
            let context = named.context.unwrap_or(named.container);
            // Convert hex string to bytes
            let user_details_bytes = hex::decode(&named.user_details)?;
            pool.execute(sqlx::query("INSERT INTO named (id, container, context, name, language, created, user_details, short_desc, long_desc, container_type) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10::container_type)")
                .bind(named.id)
                .bind(named.container)
                .bind(context)
                .bind(named.name)
                .bind(named.language)
                .bind(named.created)
                .bind(user_details_bytes)
                .bind(named.short_desc)
                .bind(named.long_desc)
                .bind(container_type)
            ).await?;
        }
        Ok(())
    }

    #[sqlx::test(migrations = "../postings-db-postgres/migrations")]
    async fn test_find_coa_by_name(pool: PgPool) -> anyhow::Result<()> {
        dotenvy::from_filename(".env.postgres").ok();
        // Arrange
        setup_data(&pool, "tests/data/coa_test_data.yml").await?;
        
        let coa_repo = Arc::new(PostgresChartOfAccountRepository::new(pool.clone()));
        let ledger_repo = Arc::new(PostgresLedgerRepository::new(pool.clone()));
        let ledger_account_repo = Arc::new(PostgresLedgerAccountRepository::new(pool.clone()));
        let named_repo = Arc::new(PostgresNamedRepository::new(pool.clone()));
        let posting_repo = Arc::new(PostgresPostingRepository::new(pool.clone()));
        let stmt_repo = Arc::new(PostgresAccountStmtRepository::new(pool.clone()));
        let line_repo = Arc::new(PostgresPostingLineRepository::new(pool.clone()));
        let trace_repo = Arc::new(PostgresPostingTraceRepository::new(pool.clone()));

        let shared_service = SharedService::new(
            coa_repo,
            ledger_repo,
            ledger_account_repo,
            named_repo,
            posting_repo,
            stmt_repo,
            line_repo,
            trace_repo,
        );
        let service = ChartOfAccountServiceImpl::new(shared_service);

        // Act
        let result = service.find_chart_of_accounts_by_name("CoA").await?;

        // Assert
        assert!(!result.is_empty(), "Should find at least one chart of account");

        Ok(())
    }
}

#[cfg(feature = "mariadb_tests")]
mod mariadb_tests {
    use std::sync::Arc;
    use serde::Deserialize;
    use sqlx::{MySqlPool, Executor};
    use uuid::Uuid;
    use hex;
    use postings_logic::services::chart_of_account_service::ChartOfAccountServiceImpl;
    use postings_api::service::chart_of_account_service::ChartOfAccountService;
    use postings_db_mariadb::repositories::chart_of_account_repository::MariaDbChartOfAccountRepository;
    use postings_db_mariadb::repositories::ledger_repository::MariaDbLedgerRepository;
    use postings_db_mariadb::repositories::ledger_account_repository::MariaDbLedgerAccountRepository;
    use postings_db_mariadb::repositories::posting_repository::MariaDbPostingRepository;
    use postings_db_mariadb::repositories::account_stmt_repository::MariaDbAccountStmtRepository;
    use postings_db_mariadb::repositories::posting_line_repository::MariaDbPostingLineRepository;
    use postings_db_mariadb::repositories::posting_trace_repository::MariaDbPostingTraceRepository;
    use postings_db_mariadb::repositories::named_repository::MariaDbNamedRepository;
    use postings_logic::services::shared_service::SharedService;

    #[derive(Deserialize)]
    struct ChartOfAccountSeed {
        id: Uuid,
    }

    #[derive(Deserialize)]
    struct NamedSeed {
        id: Uuid,
        container: Uuid,
        context: Option<Uuid>,
        name: String,
        language: String,
        created: chrono::NaiveDateTime,
        user_details: String,
        short_desc: Option<String>,
        long_desc: Option<String>,
        container_type: Option<String>,
    }

    #[derive(Deserialize)]
    struct TestDataSet {
        charts_of_account: Vec<ChartOfAccountSeed>,
        named: Vec<NamedSeed>,
    }

    async fn setup_data(pool: &MySqlPool, file_path: &str) -> anyhow::Result<()> {
        let yaml_str = std::fs::read_to_string(file_path)?;
        let data_set: TestDataSet = serde_yaml::from_str(&yaml_str)?;

        // Insert chart of accounts (just ID)
        for coa in data_set.charts_of_account {
            pool.execute(sqlx::query("INSERT INTO chart_of_account (id) VALUES (?)")
                .bind(coa.id.to_string())
            ).await?;
        }

        // Insert named entities
        for named in data_set.named {
            let container_type = named.container_type.unwrap_or("ChartOfAccount".to_string());
            let context = named.context.unwrap_or(named.container);
            // Convert hex string to bytes
            let user_details_bytes = hex::decode(&named.user_details)?;
            pool.execute(sqlx::query("INSERT INTO named (id, container, context, name, language, created, user_details, short_desc, long_desc, container_type) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
                .bind(named.id.to_string())
                .bind(named.container.to_string())
                .bind(context.to_string())
                .bind(named.name)
                .bind(named.language)
                .bind(named.created)
                .bind(user_details_bytes)
                .bind(named.short_desc)
                .bind(named.long_desc)
                .bind(container_type)
            ).await?;
        }
        Ok(())
    }

    #[sqlx::test(migrations = "../postings-db-mariadb/migrations")]
    async fn test_find_coa_by_name(pool: MySqlPool) -> anyhow::Result<()> {
        dotenvy::from_filename(".env.mariadb").ok();
        // Arrange
        setup_data(&pool, "tests/data/coa_test_data.yml").await?;
        
        let coa_repo = Arc::new(MariaDbChartOfAccountRepository::new(pool.clone()));
        let ledger_repo = Arc::new(MariaDbLedgerRepository::new(pool.clone()));
        let ledger_account_repo = Arc::new(MariaDbLedgerAccountRepository::new(pool.clone()));
        let named_repo = Arc::new(MariaDbNamedRepository::new(pool.clone()));
        let posting_repo = Arc::new(MariaDbPostingRepository::new(pool.clone()));
        let stmt_repo = Arc::new(MariaDbAccountStmtRepository::new(pool.clone()));
        let line_repo = Arc::new(MariaDbPostingLineRepository::new(pool.clone()));
        let trace_repo = Arc::new(MariaDbPostingTraceRepository::new(pool.clone()));

        let shared_service = SharedService::new(
            coa_repo,
            ledger_repo,
            ledger_account_repo,
            named_repo,
            posting_repo,
            stmt_repo,
            line_repo,
            trace_repo,
        );
        let service = ChartOfAccountServiceImpl::new(shared_service);

        // Act
        let result = service.find_chart_of_accounts_by_name("CoA").await?;

        // Assert
        assert!(!result.is_empty(), "Should find at least one chart of account");

        Ok(())
    }
}