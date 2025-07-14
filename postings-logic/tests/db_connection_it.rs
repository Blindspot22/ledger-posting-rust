#![cfg(test)]

#[cfg(feature = "mariadb_tests")]
mod mariadb_connection_tests {
    use sqlx::MySqlPool;
    use std::env;

    #[tokio::test]
    async fn test_mariadb_connection() -> anyhow::Result<()> {
        // Load environment variables from .env.mariadb
        dotenvy::from_filename(".env.mariadb").expect("Failed to load .env.mariadb file");

        // Get the database URL from environment
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set in .env.mariadb");

        println!("Attempting to connect to MariaDB at: {}", database_url);

        // Attempt to create a connection pool
        let pool = MySqlPool::connect(&database_url).await;

        // Check if the connection was successful
        match pool {
            Ok(p) => {
                println!("Successfully connected to MariaDB!");
                // Close the pool
                p.close().await;
                Ok(())
            }
            Err(e) => {
                eprintln!("Failed to connect to MariaDB: {:?}", e);
                panic!("Database connection failed: {}", e);
            }
        }
    }
}
