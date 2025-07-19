#![cfg(test)]

#[cfg(feature = "postgres_tests")]
mod postgres_tests {
    use std::sync::Arc;
    use sqlx::{PgPool, Type};
    use postings_logic::services::posting_service::PostingServiceImpl;
    use postings_api::service::posting_service::PostingService;
    use postings_db_postgres::repositories::posting_repository::PostgresPostingRepository;
    use postings_logic::services::shared_service::SharedService;
    use postings_api::domain::posting::Posting;
    use postings_api::domain::ledger::Ledger;
    use postings_api::domain::chart_of_account::ChartOfAccount;
    use uuid::Uuid;
    use bigdecimal::BigDecimal;
    use postings_api::domain::posting_line::PostingLine;
    use postings_api::domain::ledger_account::LedgerAccount;
    use postings_api::domain::balance_side::BalanceSide;
    use postings_api::domain::account_category::AccountCategory;
    use postings_db_postgres::repositories::ledger_repository::PostgresLedgerRepository;
    use postings_db_postgres::repositories::chart_of_account_repository::PostgresChartOfAccountRepository;
    use postings_db_postgres::repositories::ledger_account_repository::PostgresLedgerAccountRepository;
    use postings_db_postgres::repositories::named_repository::PostgresNamedRepository;
    use postings_db_postgres::repositories::account_stmt_repository::PostgresAccountStmtRepository;
    use postings_db_postgres::repositories::posting_line_repository::PostgresPostingLineRepository;
    use postings_db_postgres::repositories::posting_trace_repository::PostgresPostingTraceRepository;
    use postings_db::repositories::posting_line_repository::PostingLineRepository;

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


    async fn setup_ledger(pool: &PgPool) -> anyhow::Result<Ledger> {
        let coa = ChartOfAccount {
            id: Uuid::new_v4(),
        };
        
        sqlx::query("INSERT INTO chart_of_account (id) VALUES ($1)")
            .bind(coa.id)
            .execute(pool)
            .await?;

        let ledger = Ledger {
            id: Uuid::new_v4(),
            coa,
        };

        sqlx::query("INSERT INTO ledger (id, coa_id) VALUES ($1, $2)")
            .bind(ledger.id)
            .bind(ledger.coa.id)
            .execute(pool)
            .await?;
            
        Ok(ledger)
    }

    fn create_service(pool: PgPool) -> PostingServiceImpl {
        let posting_repo = Arc::new(PostgresPostingRepository::new(pool.clone()));
        let ledger_repo = Arc::new(PostgresLedgerRepository::new(pool.clone()));
        let coa_repo = Arc::new(PostgresChartOfAccountRepository::new(pool.clone()));
        let ledger_account_repo = Arc::new(PostgresLedgerAccountRepository::new(pool.clone()));
        let named_repo = Arc::new(PostgresNamedRepository::new(pool.clone()));
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
        PostingServiceImpl::new(shared_service)
    }

    struct TestContext {
        service: PostingServiceImpl,
        posting_line_repo: Arc<dyn PostingLineRepository + Send + Sync>,
    }

    fn create_test_context(
        pool: PgPool,
        line_repo: Arc<PostgresPostingLineRepository>,
    ) -> TestContext {
        let posting_repo = Arc::new(PostgresPostingRepository::new(pool.clone()));
        let ledger_repo = Arc::new(PostgresLedgerRepository::new(pool.clone()));
        let coa_repo = Arc::new(PostgresChartOfAccountRepository::new(pool.clone()));
        let ledger_account_repo = Arc::new(PostgresLedgerAccountRepository::new(pool.clone()));
        let named_repo = Arc::new(PostgresNamedRepository::new(pool.clone()));
        let stmt_repo = Arc::new(PostgresAccountStmtRepository::new(pool.clone()));
        let trace_repo = Arc::new(PostgresPostingTraceRepository::new(pool.clone()));

        let shared_service = SharedService::new(
            coa_repo,
            ledger_repo,
            ledger_account_repo,
            named_repo,
            posting_repo,
            stmt_repo,
            line_repo.clone(),
            trace_repo,
        );
        let service = PostingServiceImpl::new(shared_service);

        TestContext {
            service,
            posting_line_repo: line_repo,
        }
    }

    async fn create_test_posting(pool: &PgPool, ledger: Ledger, debit_amount: i64, credit_amount: i64) -> anyhow::Result<Posting> {
        let debit_account = setup_ledger_account(pool, &ledger, AccountCategory::AS, BalanceSide::Dr, None).await?;
        let credit_account = setup_ledger_account(pool, &ledger, AccountCategory::LI, BalanceSide::Cr, None).await?;

        Ok(Posting {
            id: Uuid::new_v4(),
            record_user: [0; 34],
            record_time: chrono::Utc::now(),
            opr_id: [0; 34],
            opr_time: chrono::Utc::now(),
            opr_type: [0; 34],
            opr_details: None,
            opr_src: None,
            pst_time: chrono::Utc::now(),
            pst_type: postings_api::domain::posting_type::PostingType::BusiTx,
            pst_status: postings_api::domain::posting_status::PostingStatus::Posted,
            ledger,
            val_time: None,
            lines: vec![
                PostingLine {
                    id: Uuid::new_v4(),
                    account: debit_account,
                    debit_amount: BigDecimal::from(debit_amount),
                    credit_amount: BigDecimal::from(0),
                    details: Some([0; 34]),
                    src_account: None,
                    base_line: None,
                    sub_opr_src_id: None,
                    record_time: chrono::Utc::now(),
                    opr_id: [0; 34],
                    opr_src: None,
                    pst_time: chrono::Utc::now(),
                    pst_type: postings_api::domain::posting_type::PostingType::BusiTx,
                    pst_status: postings_api::domain::posting_status::PostingStatus::Posted,
                    hash: Some([0; 34]),
                    additional_information: None,
                    discarded_time: None,
                },
                PostingLine {
                    id: Uuid::new_v4(),
                    account: credit_account,
                    debit_amount: BigDecimal::from(0),
                    credit_amount: BigDecimal::from(credit_amount),
                    details: Some([0; 34]),
                    src_account: None,
                    base_line: None,
                    sub_opr_src_id: None,
                    record_time: chrono::Utc::now(),
                    opr_id: [0; 34],
                    opr_src: None,
                    pst_time: chrono::Utc::now(),
                    pst_type: postings_api::domain::posting_type::PostingType::BusiTx,
                    pst_status: postings_api::domain::posting_status::PostingStatus::Posted,
                    hash: Some([0; 34]),
                    additional_information: None,
                    discarded_time: None,
                }
            ],
            discarded_id: None,
            discarded_time: None,
            discarding_id: None,
            hash_record: Default::default(),
        })
    }

    #[sqlx::test(migrations = "../postings-db-postgres/migrations")]
    async fn test_new_posting(pool: PgPool) -> anyhow::Result<()> {
        dotenvy::from_filename(".env.postgres").ok();
        // Arrange
        let ledger = setup_ledger(&pool).await?;
        let service = create_service(pool.clone());
        let posting_bo = create_test_posting(&pool, ledger, 100, 100).await?;
        let opr_id = posting_bo.opr_id.clone();

        // Act
        let result = service.new_posting(posting_bo).await?;

        // Assert
        assert_eq!(result.opr_id, opr_id);
        
        Ok(())
    }
}

#[cfg(feature = "mariadb_tests")]
mod mariadb_tests {
    use std::sync::Arc;
    use sqlx::MySqlPool;
    use postings_logic::services::posting_service::PostingServiceImpl;
    use postings_api::service::posting_service::PostingService;
    use postings_api::ServiceError;
    use postings_db_mariadb::repositories::posting_repository::MariaDbPostingRepository;
    use postings_logic::services::shared_service::SharedService;
    use postings_api::domain::posting::Posting;
    use postings_api::domain::ledger::Ledger;
    use postings_api::domain::chart_of_account::ChartOfAccount;
    use uuid::Uuid;
    use bigdecimal::BigDecimal;
    use postings_api::domain::posting_line::PostingLine;
    use postings_api::domain::ledger_account::LedgerAccount;
    use postings_api::domain::balance_side::BalanceSide;
    use postings_api::domain::account_category::AccountCategory;
    use postings_db_mariadb::repositories::ledger_repository::MariaDbLedgerRepository;
    use postings_db_mariadb::repositories::chart_of_account_repository::MariaDbChartOfAccountRepository;
    use postings_db_mariadb::repositories::ledger_account_repository::MariaDbLedgerAccountRepository;
    use postings_db_mariadb::repositories::account_stmt_repository::MariaDbAccountStmtRepository;
    use postings_db_mariadb::repositories::posting_line_repository::MariaDbPostingLineRepository;
    use postings_db_mariadb::repositories::posting_trace_repository::MariaDbPostingTraceRepository;
    use postings_db_mariadb::repositories::named_repository::MariaDbNamedRepository;
    use postings_db::repositories::posting_line_repository::PostingLineRepository;

    async fn setup_ledger_account(pool: &MySqlPool, ledger: &Ledger, name: &str, category: AccountCategory, balance_side: BalanceSide, parent: Option<&LedgerAccount>) -> anyhow::Result<LedgerAccount> {
        let ledger_account_id = Uuid::new_v4();
        let ledger_account = LedgerAccount {
            id: ledger_account_id,
            ledger: ledger.clone(),
            parent: parent.map(|p| Box::new(p.clone())),
            coa: ledger.coa.clone(),
            balance_side,
            category,
        };
        
        // Insert into simplified ledger_account table
        sqlx::query("INSERT INTO ledger_account (id, ledger_id, parent_id, coa_id, balance_side, category) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(ledger_account_id.to_string())
            .bind(ledger_account.ledger.id.to_string())
            .bind(parent.map(|p| p.id.to_string()))
            .bind(ledger_account.coa.id.to_string())
            .bind(match ledger_account.balance_side {
                BalanceSide::Dr => "Dr",
                BalanceSide::Cr => "Cr",
                BalanceSide::DrCr => "DrCr",
            })
            .bind(match ledger_account.category {
                AccountCategory::RE => "RE",
                AccountCategory::EX => "EX",
                AccountCategory::AS => "AS",
                AccountCategory::LI => "LI",
                AccountCategory::EQ => "EQ",
                AccountCategory::NOOP => "NOOP",
                AccountCategory::NORE => "NORE",
                AccountCategory::NOEX => "NOEX",
            })
            .execute(pool)
            .await?;
        
        // Insert named entity for the ledger account
        let user_details_bytes = [0u8; 34]; // Create proper 34-byte array
        sqlx::query("INSERT INTO named (id, container, context, name, language, created, user_details, short_desc, long_desc, container_type) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(Uuid::new_v4().to_string())
            .bind(ledger_account_id.to_string())
            .bind(ledger_account.coa.id.to_string())
            .bind(name)
            .bind("en")
            .bind(chrono::Utc::now())
            .bind(&user_details_bytes[..])
            .bind(None::<String>)
            .bind(None::<String>)
            .bind("LedgerAccount")
            .execute(pool)
            .await?;

        Ok(ledger_account)
    }


    async fn setup_ledger(pool: &MySqlPool) -> anyhow::Result<Ledger> {
        let coa_id = Uuid::new_v4();
        let coa = ChartOfAccount {
            id: coa_id,
        };
        
        // Insert into simplified chart_of_account table
        sqlx::query("INSERT INTO chart_of_account (id) VALUES (?)")
            .bind(coa_id.to_string())
            .execute(pool)
            .await?;
        
        // Insert named entity for COA
        let user_details_bytes = [0u8; 34];
        sqlx::query("INSERT INTO named (id, container, context, name, language, created, user_details, short_desc, long_desc, container_type) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(Uuid::new_v4().to_string())
            .bind(coa_id.to_string())
            .bind(coa_id.to_string())
            .bind("Test COA")
            .bind("en")
            .bind(chrono::Utc::now())
            .bind(&user_details_bytes[..])
            .bind(Some("Short desc"))
            .bind(Some("Long desc"))
            .bind("ChartOfAccount")
            .execute(pool)
            .await?;

        let ledger_id = Uuid::new_v4();
        let ledger = Ledger {
            id: ledger_id,
            coa,
        };

        // Insert into simplified ledger table
        sqlx::query("INSERT INTO ledger (id, coa_id) VALUES (?, ?)")
            .bind(ledger_id.to_string())
            .bind(ledger.coa.id.to_string())
            .execute(pool)
            .await?;
        
        // Insert named entity for ledger
        sqlx::query("INSERT INTO named (id, container, context, name, language, created, user_details, short_desc, long_desc, container_type) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(Uuid::new_v4().to_string())
            .bind(ledger_id.to_string())
            .bind(coa_id.to_string())
            .bind("Test Ledger")
            .bind("en")
            .bind(chrono::Utc::now())
            .bind(&user_details_bytes[..])
            .bind(Some("Short desc"))
            .bind(Some("Long desc"))
            .bind("Ledger")
            .execute(pool)
            .await?;
            
        Ok(ledger)
    }

    fn create_service(pool: MySqlPool) -> PostingServiceImpl {
        let posting_repo = Arc::new(MariaDbPostingRepository::new(pool.clone()));
        let ledger_repo = Arc::new(MariaDbLedgerRepository::new(pool.clone()));
        let coa_repo = Arc::new(MariaDbChartOfAccountRepository::new(pool.clone()));
        let ledger_account_repo = Arc::new(MariaDbLedgerAccountRepository::new(pool.clone()));
        let named_repo = Arc::new(MariaDbNamedRepository::new(pool.clone()));
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
        PostingServiceImpl::new(shared_service)
    }

    struct TestContext {
        service: PostingServiceImpl,
        posting_line_repo: Arc<dyn PostingLineRepository + Send + Sync>,
    }

    fn create_test_context(
        pool: MySqlPool,
        line_repo: Arc<MariaDbPostingLineRepository>,
    ) -> TestContext {
        let posting_repo = Arc::new(MariaDbPostingRepository::new(pool.clone()));
        let ledger_repo = Arc::new(MariaDbLedgerRepository::new(pool.clone()));
        let coa_repo = Arc::new(MariaDbChartOfAccountRepository::new(pool.clone()));
        let ledger_account_repo = Arc::new(MariaDbLedgerAccountRepository::new(pool.clone()));
        let named_repo = Arc::new(MariaDbNamedRepository::new(pool.clone()));
        let stmt_repo = Arc::new(MariaDbAccountStmtRepository::new(pool.clone()));
        let trace_repo = Arc::new(MariaDbPostingTraceRepository::new(pool.clone()));

        let shared_service = SharedService::new(
            coa_repo,
            ledger_repo,
            ledger_account_repo,
            named_repo,
            posting_repo,
            stmt_repo,
            line_repo.clone(),
            trace_repo,
        );
        let service = PostingServiceImpl::new(shared_service);

        TestContext {
            service,
            posting_line_repo: line_repo,
        }
    }

    async fn create_test_posting(pool: &MySqlPool, ledger: Ledger, debit_amount: i64, credit_amount: i64) -> anyhow::Result<Posting> {
        let debit_account = setup_ledger_account(pool, &ledger, &format!("Debit Account {}", Uuid::new_v4()), AccountCategory::AS, BalanceSide::Dr, None).await?;
        let credit_account = setup_ledger_account(pool, &ledger, &format!("Credit Account {}", Uuid::new_v4()), AccountCategory::LI, BalanceSide::Cr, None).await?;

        Ok(Posting {
            id: Uuid::new_v4(),
            record_user: [0; 34],
            record_time: chrono::Utc::now(),
            opr_id: [1; 34],
            opr_time: chrono::Utc::now(),
            opr_type: [2; 34],
            opr_details: Some([3; 34]),
            opr_src: None,
            pst_time: chrono::Utc::now(),
            pst_type: postings_api::domain::posting_type::PostingType::BusiTx,
            pst_status: postings_api::domain::posting_status::PostingStatus::Posted,
            ledger,
            val_time: None,
            lines: vec![
                PostingLine {
                    id: Uuid::new_v4(),
                    account: debit_account,
                    debit_amount: BigDecimal::from(debit_amount),
                    credit_amount: BigDecimal::from(0),
                    details: Some([4; 34]),
                    src_account: None,
                    base_line: None,
                    sub_opr_src_id: None,
                    record_time: chrono::Utc::now(),
                    opr_id: [5; 34],
                    opr_src: None,
                    pst_time: chrono::Utc::now(),
                    pst_type: postings_api::domain::posting_type::PostingType::BusiTx,
                    pst_status: postings_api::domain::posting_status::PostingStatus::Posted,
                    hash: Some([1; 34]),
                    additional_information: None,
                    discarded_time: None,
                },
                PostingLine {
                    id: Uuid::new_v4(),
                    account: credit_account,
                    debit_amount: BigDecimal::from(0),
                    credit_amount: BigDecimal::from(credit_amount),
                    details: Some([6; 34]),
                    src_account: None,
                    base_line: None,
                    sub_opr_src_id: None,
                    record_time: chrono::Utc::now(),
                    opr_id: [7; 34],
                    opr_src: None,
                    pst_time: chrono::Utc::now(),
                    pst_type: postings_api::domain::posting_type::PostingType::BusiTx,
                    pst_status: postings_api::domain::posting_status::PostingStatus::Posted,
                    hash: Some([2; 34]),
                    additional_information: None,
                    discarded_time: None,
                }
            ],
            discarded_id: None,
            discarded_time: None,
            discarding_id: None,
            hash_record: Default::default(),
        })
    }

    #[sqlx::test(migrations = "../postings-db-mariadb/migrations")]
    async fn test_new_posting(pool: MySqlPool) -> anyhow::Result<()> {
        dotenvy::from_filename(".env.mariadb").ok();
        // Arrange
        let ledger = setup_ledger(&pool).await?;
        let service = create_service(pool.clone());
        let posting_bo = create_test_posting(&pool, ledger, 100, 100).await?;
        let opr_id = posting_bo.opr_id.clone();

        // Act
        let result = service.new_posting(posting_bo).await?;

        // Assert
        assert_eq!(result.opr_id, opr_id);
        
        Ok(())
    }

    #[sqlx::test(migrations = "../postings-db-mariadb/migrations")]
    async fn test_new_posting_unbalanced_fails(pool: MySqlPool) -> anyhow::Result<()> {
        dotenvy::from_filename(".env.mariadb").ok();
        // Arrange
        let ledger = setup_ledger(&pool).await?;
        let service = create_service(pool.clone());
        let posting_bo = create_test_posting(&pool, ledger, 100, 99).await?; // Unbalanced

        // Act
        let result = service.new_posting(posting_bo).await;

        // Assert
        assert!(matches!(result, Err(ServiceError::DoubleEntry)));

        Ok(())
    }

    #[sqlx::test(migrations = "../postings-db-mariadb/migrations")]
    async fn test_new_posting_sets_antecedent_hash(pool: MySqlPool) -> anyhow::Result<()> {
        dotenvy::from_filename(".env.mariadb").ok();
        // Arrange
        let ledger = setup_ledger(&pool).await?;
        let service = create_service(pool.clone());

        // First posting
        let posting_bo1 = create_test_posting(&pool, ledger.clone(), 100, 100).await?;
        let result1 = service.new_posting(posting_bo1).await?;

        // Assert first posting
        assert!(result1.hash_record.antecedent_id.is_none());
        assert!(result1.hash_record.antecedent_hash.is_none());
        assert!(result1.hash_record.hash.is_some());

        // Give some time to ensure record_time is different
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        // Second posting
        let posting_bo2 = create_test_posting(&pool, ledger, 200, 200).await?;
        let result2 = service.new_posting(posting_bo2).await?;

        // Assert second posting
        assert_eq!(result2.hash_record.antecedent_id, Some(result1.id));
        assert_eq!(result2.hash_record.antecedent_hash, result1.hash_record.hash);
        assert!(result2.hash_record.hash.is_some());
        assert_ne!(result1.hash_record.hash, result2.hash_record.hash);

        Ok(())
    }

    #[sqlx::test(migrations = "../postings-db-mariadb/migrations")]
    async fn test_new_posting_stores_lines(pool: MySqlPool) -> anyhow::Result<()> {
        dotenvy::from_filename(".env.mariadb").ok();
        // Arrange
        let ledger = setup_ledger(&pool).await?;
        let line_repo = Arc::new(MariaDbPostingLineRepository::new(pool.clone()));
        let context = create_test_context(pool.clone(), line_repo.clone());
        let posting_bo = create_test_posting(&pool, ledger, 100, 100).await?;
        let line_ids: Vec<Uuid> = posting_bo.lines.iter().map(|l| l.id).collect();

        // Act
        context.service.new_posting(posting_bo).await?;

        // Assert
        for line_id in line_ids {
            let stored_line = context.posting_line_repo.find_by_id(line_id).await?
                .expect("Posting line not found");
            assert_eq!(stored_line.id, line_id);
        }
        
        Ok(())
    }

    #[sqlx::test(migrations = "../postings-db-mariadb/migrations")]
    async fn test_new_posting_stores_line_details(pool: MySqlPool) -> anyhow::Result<()> {
        dotenvy::from_filename(".env.mariadb").ok();
        // Arrange
        let ledger = setup_ledger(&pool).await?;
        let line_repo = Arc::new(MariaDbPostingLineRepository::new(pool.clone()));
        let context = create_test_context(pool.clone(), line_repo.clone());
        let posting_bo = create_test_posting(&pool, ledger, 100, 100).await?;
        let expected_details: Vec<Option<[u8; 34]>> = posting_bo.lines.iter().map(|l| l.details.clone()).collect();
        let line_ids: Vec<Uuid> = posting_bo.lines.iter().map(|l| l.id).collect();

        // Act
        context.service.new_posting(posting_bo).await?;

        // Assert
        for (line_id, expected_detail) in line_ids.iter().zip(expected_details.iter()) {
            let stored_line = context.posting_line_repo.find_by_id(*line_id).await?
                .expect("Posting line not found");
            
            assert_eq!(&stored_line.details, expected_detail);
        }
        
        Ok(())
    }
}