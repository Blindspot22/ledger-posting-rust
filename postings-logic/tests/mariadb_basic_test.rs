#![cfg(test)]

#[cfg(feature = "mariadb_tests")]
mod mariadb_basic_tests {
    use sqlx::MySqlPool;
    use std::env;
    use postings_db_mariadb::repositories::posting_repository::MariaDbPostingRepository;
    use postings_db_mariadb::repositories::posting_line_repository::MariaDbPostingLineRepository;
    use postings_db_mariadb::repositories::posting_trace_repository::MariaDbPostingTraceRepository;
    use postings_db_mariadb::repositories::chart_of_account_repository::MariaDbChartOfAccountRepository;
    use postings_db_mariadb::repositories::ledger_repository::MariaDbLedgerRepository;
    use postings_db_mariadb::repositories::ledger_account_repository::MariaDbLedgerAccountRepository;
    use postings_db_mariadb::repositories::named_repository::MariaDbNamedRepository;

    #[tokio::test]
    async fn test_mariadb_repository_creation() -> anyhow::Result<()> {
        // Load environment variables from .env.mariadb
        dotenvy::from_filename(".env.mariadb").ok();

        // Get the database URL from environment
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        println!("Testing MariaDB repository creation with: {}", database_url);

        // Create connection pool
        let pool = MySqlPool::connect(&database_url).await?;

        // Test that all repositories can be created
        let _posting_repo = MariaDbPostingRepository::new(pool.clone());
        let _posting_line_repo = MariaDbPostingLineRepository::new(pool.clone());
        let _posting_trace_repo = MariaDbPostingTraceRepository::new(pool.clone());
        let _coa_repo = MariaDbChartOfAccountRepository::new(pool.clone());
        let _ledger_repo = MariaDbLedgerRepository::new(pool.clone());
        let _ledger_account_repo = MariaDbLedgerAccountRepository::new(pool.clone());
        let _named_repo = MariaDbNamedRepository::new(pool.clone());

        println!("All MariaDB repositories created successfully!");

        // Close the pool
        pool.close().await;
        Ok(())
    }

    #[tokio::test]
    async fn test_mariadb_schema_migration() -> anyhow::Result<()> {
        // Load environment variables from .env.mariadb
        dotenvy::from_filename(".env.mariadb").ok();

        // Get the database URL from environment
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        println!("Testing MariaDB schema migration with: {}", database_url);

        // Create connection pool
        let pool = MySqlPool::connect(&database_url).await?;

        // Run migrations
        sqlx::migrate!("../postings-db-mariadb/migrations")
            .run(&pool)
            .await?;

        println!("MariaDB schema migrations applied successfully!");

        // Test that we can query basic table structure
        let result = sqlx::query("SHOW TABLES")
            .fetch_all(&pool)
            .await?;

        println!("Found {} tables in database", result.len());

        // Close the pool
        pool.close().await;
        Ok(())
    }
}