#![cfg(test)]

#[cfg(feature = "mariadb_tests")]
mod mariadb_debug_tests {
    use sqlx::{MySqlPool, Executor};
    use uuid::Uuid;
    use std::sync::Arc;
    use postings_db_mariadb::repositories::chart_of_account_repository::MariaDbChartOfAccountRepository;
    use postings_db_mariadb::repositories::named_repository::MariaDbNamedRepository;
    use postings_db::models::named::ContainerType;
    use postings_db::repositories::named_repository::NamedRepository;
    // use postings_db::repositories::chart_of_account_repository::ChartOfAccountRepository;

    #[sqlx::test(migrations = "../postings-db-mariadb/migrations")]
    async fn test_debug_named_repository(pool: MySqlPool) -> anyhow::Result<()> {
        // Create a simple chart of account
        let coa_id = Uuid::parse_str("11111111-1111-1111-1111-111111111111")?;
        let _coa_repo = Arc::new(MariaDbChartOfAccountRepository::new(pool.clone()));

        // Insert chart of account manually
        pool.execute(sqlx::query("INSERT INTO chart_of_account (id) VALUES (?)")
            .bind(coa_id.to_string())
        ).await?;

        // Insert named entity manually with proper data
        let named_id = Uuid::parse_str("21111111-1111-1111-1111-111111111111")?;
        let user_details_bytes = hex::decode("00000000000000000000000000000000000000000000000000000000000000000000")?;
        
        pool.execute(sqlx::query("INSERT INTO named (id, container, context, name, language, created, user_details, short_desc, long_desc, container_type) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(named_id.to_string())
            .bind(coa_id.to_string())
            .bind(coa_id.to_string())
            .bind("CoA")
            .bind("en")
            .bind(chrono::NaiveDateTime::parse_from_str("2018-08-07 20:58:24.232", "%Y-%m-%d %H:%M:%S%.3f")?)
            .bind(user_details_bytes)
            .bind("Sample chart of account")
            .bind(None::<String>)
            .bind("ChartOfAccount")
        ).await?;

        // Test the named repository
        let named_repo = Arc::new(MariaDbNamedRepository::new(pool.clone()));
        
        println!("Testing named repository find_by_name_and_type...");
        let result = named_repo.find_by_name_and_type("CoA", ContainerType::ChartOfAccount).await;
        
        match result {
            Ok(named_entries) => {
                println!("Found {} named entries", named_entries.len());
                for entry in &named_entries {
                    println!("Named entry: id={}, name={}, container={}", entry.id, entry.name, entry.container);
                }
            },
            Err(e) => {
                println!("Error in named repository: {:?}", e);
                return Err(anyhow::anyhow!("Named repository error: {:?}", e));
            }
        }

        Ok(())
    }
}