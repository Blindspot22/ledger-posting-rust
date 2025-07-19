#![cfg(test)]

#[cfg(feature = "postgres_tests")]
mod postgres_tests {
    use std::sync::Arc;
    use sqlx::{PgPool, Type};
    use postings_logic::services::ledger_service::LedgerServiceImpl;
    use postings_logic::services::chart_of_account_service::ChartOfAccountServiceImpl;
    use postings_api::service::ledger_service::LedgerService;
    use postings_db_postgres::repositories::ledger_repository::PostgresLedgerRepository;
    use postings_db_postgres::repositories::chart_of_account_repository::PostgresChartOfAccountRepository;
    use postings_db_postgres::repositories::ledger_account_repository::PostgresLedgerAccountRepository;
    use postings_db_postgres::repositories::named_repository::PostgresNamedRepository;
    use postings_db_postgres::repositories::posting_repository::PostgresPostingRepository;
    use postings_db_postgres::repositories::account_stmt_repository::PostgresAccountStmtRepository;
    use postings_db_postgres::repositories::posting_line_repository::PostgresPostingLineRepository;
    use postings_db_postgres::repositories::posting_trace_repository::PostgresPostingTraceRepository;
    use postings_logic::services::shared_service::SharedService;
    use postings_api::domain::ledger::Ledger;
    use postings_api::domain::chart_of_account::ChartOfAccount;
    use postings_api::domain::named::{Named, ContainerType};
    use uuid::Uuid;
    use postings_api::domain::{
        account_category::AccountCategory, balance_side::BalanceSide, ledger_account::LedgerAccount,
    };

    #[derive(Type)]
    #[sqlx(type_name = "balance_side")]
    pub enum TestBalanceSide {
        Dr,
        Cr,
        DrCr,
    }

    #[derive(Type)]
    #[sqlx(type_name = "account_category")]
    pub enum TestAccountCategory {
        RE,
        EX,
        AS,
        LI,
        EQ,
        NOOP,
        NORE,
        NOEX,
    }

    async fn setup_coa(pool: &PgPool) -> anyhow::Result<ChartOfAccount> {
        let coa = ChartOfAccount {
            id: Uuid::new_v4(),
        };
        
        sqlx::query("INSERT INTO chart_of_account (id) VALUES ($1)")
            .bind(coa.id)
            .execute(pool)
            .await?;
            
        Ok(coa)
    }

    async fn setup_ledger(pool: &PgPool, coa: &ChartOfAccount) -> anyhow::Result<Ledger> {
        let ledger = Ledger {
            id: Uuid::new_v4(),
            coa: coa.clone(),
        };
        sqlx::query("INSERT INTO ledger (id, coa_id) VALUES ($1, $2)")
            .bind(ledger.id)
            .bind(ledger.coa.id)
            .execute(pool)
            .await?;
        Ok(ledger)
    }

    async fn setup_ledger_account(pool: &PgPool, ledger: &Ledger, category: AccountCategory, balance_side: BalanceSide, parent: Option<&LedgerAccount>) -> anyhow::Result<LedgerAccount> {
        let ledger_account = LedgerAccount {
            id: Uuid::new_v4(),
            ledger: ledger.clone(),
            parent: parent.map(|p| Box::new(p.clone())),
            coa: ledger.coa.clone(),
            balance_side,
            category,
        };
        sqlx::query("INSERT INTO ledger_account (id, ledger_id, parent_id, coa_id, balance_side, category) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(ledger_account.id)
            .bind(ledger_account.ledger.id)
            .bind(parent.map(|p| p.id))
            .bind(ledger_account.coa.id)
            .bind(match ledger_account.balance_side {
                BalanceSide::Dr => TestBalanceSide::Dr,
                BalanceSide::Cr => TestBalanceSide::Cr,
                BalanceSide::DrCr => TestBalanceSide::DrCr,
            })
            .bind(match ledger_account.category {
                AccountCategory::RE => TestAccountCategory::RE,
                AccountCategory::EX => TestAccountCategory::EX,
                AccountCategory::AS => TestAccountCategory::AS,
                AccountCategory::LI => TestAccountCategory::LI,
                AccountCategory::EQ => TestAccountCategory::EQ,
                AccountCategory::NOOP => TestAccountCategory::NOOP,
                AccountCategory::NORE => TestAccountCategory::NORE,
                AccountCategory::NOEX => TestAccountCategory::NOEX,
            })
            .execute(pool)
            .await?;

        Ok(ledger_account)
    }

    fn create_service(pool: PgPool) -> LedgerServiceImpl {
        let coa_repo = Arc::new(PostgresChartOfAccountRepository::new(pool.clone()));
        let ledger_repo = Arc::new(PostgresLedgerRepository::new(pool.clone()));
        let ledger_account_repo = Arc::new(PostgresLedgerAccountRepository::new(pool.clone()));
        let named_repo = Arc::new(PostgresNamedRepository::new(pool.clone()));
        let posting_repo = Arc::new(PostgresPostingRepository::new(pool.clone()));
        let stmt_repo = Arc::new(PostgresAccountStmtRepository::new(pool.clone()));
        let line_repo = Arc::new(PostgresPostingLineRepository::new(pool.clone()));
        let trace_repo = Arc::new(PostgresPostingTraceRepository::new(pool.clone()));

        let shared_service = SharedService::new(
            coa_repo.clone(),
            ledger_repo,
            ledger_account_repo,
            named_repo.clone(),
            posting_repo,
            stmt_repo,
            line_repo,
            trace_repo,
        );
        let coa_service = ChartOfAccountServiceImpl::new(SharedService::new(
            coa_repo,
            Arc::new(PostgresLedgerRepository::new(pool.clone())),
            Arc::new(PostgresLedgerAccountRepository::new(pool.clone())),
            named_repo,
            Arc::new(PostgresPostingRepository::new(pool.clone())),
            Arc::new(PostgresAccountStmtRepository::new(pool.clone())),
            Arc::new(PostgresPostingLineRepository::new(pool.clone())),
            Arc::new(PostgresPostingTraceRepository::new(pool.clone())),
        ));
        LedgerServiceImpl::new(shared_service, coa_service)
    }

    #[sqlx::test(migrations = "../postings-db-postgres/migrations")]
    async fn test_new_ledger(pool: PgPool) -> anyhow::Result<()> {
        dotenvy::from_filename(".env.postgres").ok();
        // Arrange
        let coa = setup_coa(&pool).await?;
        let service = create_service(pool);

        let ledger_bo = Ledger {
            id: Uuid::new_v4(),
            coa,
        };
        let named_bo = vec![Named {
            id: Uuid::new_v4(),
            container: ledger_bo.id,
            context: ledger_bo.coa.id,
            name: "Test Ledger".to_string(),
            language: "en".to_string(),
            created: chrono::Utc::now(),
            user_details: [0; 34],
            short_desc: Some("Short desc".to_string()),
            long_desc: Some("Long desc".to_string()),
            container_type: ContainerType::Ledger,
        }];

        // Act
        let (result, named_result) = service.new_ledger(ledger_bo, named_bo).await?;

        // Assert
        assert_eq!(named_result[0].name, "Test Ledger");
        
        Ok(())
    }
}
