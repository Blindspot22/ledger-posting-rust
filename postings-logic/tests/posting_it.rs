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
    use postings_api::domain::named::Named;
    use uuid::Uuid;
    use bigdecimal::BigDecimal;
    use postings_api::domain::posting_line::PostingLine;
    use postings_api::domain::ledger_account::LedgerAccount;
    use postings_api::domain::balance_side::BalanceSide;
    use postings_api::domain::account_category::AccountCategory;
    use postings_db_postgres::repositories::ledger_repository::PostgresLedgerRepository;
    use postings_db_postgres::repositories::chart_of_account_repository::PostgresChartOfAccountRepository;
    use postings_db_postgres::repositories::ledger_account_repository::PostgresLedgerAccountRepository;
    use postings_db_postgres::repositories::account_stmt_repository::PostgresAccountStmtRepository;
    use postings_db_postgres::repositories::posting_line_repository::PostgresPostingLineRepository;
    use postings_db_postgres::repositories::posting_trace_repository::PostgresPostingTraceRepository;
    use postings_api::ServiceError;
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

    async fn setup_ledger_account(pool: &PgPool, ledger: &Ledger, name: &str, category: AccountCategory, balance_side: BalanceSide, parent: Option<&LedgerAccount>) -> anyhow::Result<LedgerAccount> {
        let ledger_account = LedgerAccount {
            named: Named {
                id: Uuid::new_v4(),
                name: name.to_string(),
                created: chrono::Utc::now(),
                user_details: "test_user".to_string(),
                short_desc: None,
                long_desc: None,
            },
            ledger: ledger.clone(),
            parent: parent.map(|p| Box::new(p.clone())),
            coa: ledger.coa.clone(),
            balance_side,
            category,
        };
        sqlx::query("INSERT INTO ledger_account (id, name, ledger_id, parent_id, coa_id, balance_side, category, created, user_details) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)")
            .bind(ledger_account.named.id)
            .bind(&ledger_account.named.name)
            .bind(ledger_account.ledger.named.id)
            .bind(parent.map(|p| p.named.id))
            .bind(ledger_account.coa.named.id)
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
            .bind(ledger_account.named.created)
            .bind(&ledger_account.named.user_details)
            .execute(pool)
            .await?;

        Ok(ledger_account)
    }


    async fn setup_ledger(pool: &PgPool) -> anyhow::Result<Ledger> {
        let coa = ChartOfAccount {
            named: Named {
                id: Uuid::new_v4(),
                name: "Test COA".to_string(),
                created: chrono::Utc::now(),
                user_details: "test_user".to_string(),
                short_desc: Some("Short desc".to_string()),
                long_desc: Some("Long desc".to_string()),
            }
        };
        
        sqlx::query("INSERT INTO chart_of_account (id, name, created, user_details, short_desc, long_desc) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(coa.named.id)
            .bind(&coa.named.name)
            .bind(coa.named.created)
            .bind(&coa.named.user_details)
            .bind(&coa.named.short_desc)
            .bind(&coa.named.long_desc)
            .execute(pool)
            .await?;

        let ledger = Ledger {
            named: Named {
                id: Uuid::new_v4(),
                name: "Test Ledger".to_string(),
                created: chrono::Utc::now(),
                user_details: "test_user".to_string(),
                short_desc: Some("Short desc".to_string()),
                long_desc: Some("Long desc".to_string()),
            },
            coa,
        };

        sqlx::query("INSERT INTO ledger (id, name, coa_id, created, user_details, short_desc, long_desc) VALUES ($1, $2, $3, $4, $5, $6, $7)")
            .bind(ledger.named.id)
            .bind(&ledger.named.name)
            .bind(ledger.coa.named.id)
            .bind(ledger.named.created)
            .bind(&ledger.named.user_details)
            .bind(&ledger.named.short_desc)
            .bind(&ledger.named.long_desc)
            .execute(pool)
            .await?;
            
        Ok(ledger)
    }

    fn create_service(pool: PgPool) -> PostingServiceImpl {
        let posting_repo = Arc::new(PostgresPostingRepository::new(pool.clone()));
        let ledger_repo = Arc::new(PostgresLedgerRepository::new(pool.clone()));
        let coa_repo = Arc::new(PostgresChartOfAccountRepository::new(pool.clone()));
        let ledger_account_repo = Arc::new(PostgresLedgerAccountRepository::new(pool.clone()));
        let stmt_repo = Arc::new(PostgresAccountStmtRepository::new(pool.clone()));
        let line_repo = Arc::new(PostgresPostingLineRepository::new(pool.clone()));
        let trace_repo = Arc::new(PostgresPostingTraceRepository::new(pool.clone()));

        let shared_service = SharedService::new(
            coa_repo,
            ledger_repo,
            ledger_account_repo,
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
        let stmt_repo = Arc::new(PostgresAccountStmtRepository::new(pool.clone()));
        let trace_repo = Arc::new(PostgresPostingTraceRepository::new(pool.clone()));

        let shared_service = SharedService::new(
            coa_repo,
            ledger_repo,
            ledger_account_repo,
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
        let debit_account = setup_ledger_account(pool, &ledger, &format!("Debit Account {}", Uuid::new_v4()), AccountCategory::AS, BalanceSide::Dr, None).await?;
        let credit_account = setup_ledger_account(pool, &ledger, &format!("Credit Account {}", Uuid::new_v4()), AccountCategory::LI, BalanceSide::Cr, None).await?;

        Ok(Posting {
            id: Uuid::new_v4(),
            record_user: "test_user".to_string(),
            record_time: chrono::Utc::now(),
            opr_id: format!("test_opr_{}", Uuid::new_v4()),
            opr_time: chrono::Utc::now(),
            opr_type: "test_type".to_string(),
            opr_details: Some("test_details".to_string()),
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
                    details: Some("debit".to_string()),
                    src_account: None,
                    base_line: None,
                    sub_opr_src_id: None,
                    record_time: chrono::Utc::now(),
                    opr_id: "test_opr".to_string(),
                    opr_src: None,
                    pst_time: chrono::Utc::now(),
                    pst_type: postings_api::domain::posting_type::PostingType::BusiTx,
                    pst_status: postings_api::domain::posting_status::PostingStatus::Posted,
                    hash: "hash".to_string(),
                    additional_information: None,
                    discarded_time: None,
                },
                PostingLine {
                    id: Uuid::new_v4(),
                    account: credit_account,
                    debit_amount: BigDecimal::from(0),
                    credit_amount: BigDecimal::from(credit_amount),
                    details: Some("credit".to_string()),
                    src_account: None,
                    base_line: None,
                    sub_opr_src_id: None,
                    record_time: chrono::Utc::now(),
                    opr_id: "test_opr".to_string(),
                    opr_src: None,
                    pst_time: chrono::Utc::now(),
                    pst_type: postings_api::domain::posting_type::PostingType::BusiTx,
                    pst_status: postings_api::domain::posting_status::PostingStatus::Posted,
                    hash: "hash".to_string(),
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

    #[sqlx::test(migrations = "../postings-db-postgres/migrations")]
    async fn test_new_posting_unbalanced_fails(pool: PgPool) -> anyhow::Result<()> {
        dotenvy::from_filename(".env.postgres").ok();
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

    #[sqlx::test(migrations = "../postings-db-postgres/migrations")]
    async fn test_new_posting_sets_antecedent_hash(pool: PgPool) -> anyhow::Result<()> {
        dotenvy::from_filename(".env.postgres").ok();
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

    #[sqlx::test(migrations = "../postings-db-postgres/migrations")]
    async fn test_new_posting_stores_lines(pool: PgPool) -> anyhow::Result<()> {
        dotenvy::from_filename(".env.postgres").ok();
        // Arrange
        let ledger = setup_ledger(&pool).await?;
        let line_repo = Arc::new(PostgresPostingLineRepository::new(pool.clone()));
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

    #[sqlx::test(migrations = "../postings-db-postgres/migrations")]
    async fn test_new_posting_stores_line_details(pool: PgPool) -> anyhow::Result<()> {
        dotenvy::from_filename(".env.postgres").ok();
        // Arrange
        let ledger = setup_ledger(&pool).await?;
        let line_repo = Arc::new(PostgresPostingLineRepository::new(pool.clone()));
        let context = create_test_context(pool.clone(), line_repo.clone());
        let posting_bo = create_test_posting(&pool, ledger, 100, 100).await?;
        let expected_details: Vec<Option<String>> = posting_bo.lines.iter().map(|l| l.details.clone()).collect();
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

#[cfg(feature = "mariadb_tests")]
mod mariadb_tests {
    use std::sync::Arc;
    use sqlx::MySqlPool;
    use postings_logic::services::posting_service::PostingServiceImpl;
    use postings_api::service::posting_service::PostingService;
    use postings_db_mariadb::repositories::posting_repository::MariaDbPostingRepository;
    use postings_logic::services::shared_service::SharedService;
    use postings_api::domain::posting::Posting;
    use postings_api::domain::ledger::Ledger;
    use postings_api::domain::chart_of_account::ChartOfAccount;
    use postings_api::domain::named::Named;
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
    use postings_api::ServiceError;
    use postings_db::repositories::posting_line_repository::PostingLineRepository;

    async fn setup_ledger_account(pool: &MySqlPool, ledger: &Ledger, name: &str, category: AccountCategory, balance_side: BalanceSide, parent: Option<&LedgerAccount>) -> anyhow::Result<LedgerAccount> {
        let ledger_account = LedgerAccount {
            named: Named {
                id: Uuid::new_v4(),
                name: name.to_string(),
                created: chrono::Utc::now(),
                user_details: "test_user".to_string(),
                short_desc: None,
                long_desc: None,
            },
            ledger: ledger.clone(),
            parent: parent.map(|p| Box::new(p.clone())),
            coa: ledger.coa.clone(),
            balance_side,
            category,
        };
        sqlx::query("INSERT INTO ledger_account (id, name, ledger_id, parent_id, coa_id, balance_side, category, created, user_details) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(ledger_account.named.id.to_string())
            .bind(&ledger_account.named.name)
            .bind(ledger_account.ledger.named.id.to_string())
            .bind(parent.map(|p| p.named.id.to_string()))
            .bind(ledger_account.coa.named.id.to_string())
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
            .bind(ledger_account.named.created)
            .bind(&ledger_account.named.user_details)
            .execute(pool)
            .await?;

        Ok(ledger_account)
    }


    async fn setup_ledger(pool: &MySqlPool) -> anyhow::Result<Ledger> {
        let coa = ChartOfAccount {
            named: Named {
                id: Uuid::new_v4(),
                name: "Test COA".to_string(),
                created: chrono::Utc::now(),
                user_details: "test_user".to_string(),
                short_desc: Some("Short desc".to_string()),
                long_desc: Some("Long desc".to_string()),
            }
        };
        
        sqlx::query("INSERT INTO chart_of_account (id, name, created, user_details, short_desc, long_desc) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(coa.named.id.to_string())
            .bind(&coa.named.name)
            .bind(coa.named.created)
            .bind(&coa.named.user_details)
            .bind(&coa.named.short_desc)
            .bind(&coa.named.long_desc)
            .execute(pool)
            .await?;

        let ledger = Ledger {
            named: Named {
                id: Uuid::new_v4(),
                name: "Test Ledger".to_string(),
                created: chrono::Utc::now(),
                user_details: "test_user".to_string(),
                short_desc: Some("Short desc".to_string()),
                long_desc: Some("Long desc".to_string()),
            },
            coa,
        };

        sqlx::query("INSERT INTO ledger (id, name, coa_id, created, user_details, short_desc, long_desc) VALUES (?, ?, ?, ?, ?, ?, ?)")
            .bind(ledger.named.id.to_string())
            .bind(&ledger.named.name)
            .bind(ledger.coa.named.id.to_string())
            .bind(ledger.named.created)
            .bind(&ledger.named.user_details)
            .bind(&ledger.named.short_desc)
            .bind(&ledger.named.long_desc)
            .execute(pool)
            .await?;
            
        Ok(ledger)
    }

    fn create_service(pool: MySqlPool) -> PostingServiceImpl {
        let posting_repo = Arc::new(MariaDbPostingRepository::new(pool.clone()));
        let ledger_repo = Arc::new(MariaDbLedgerRepository::new(pool.clone()));
        let coa_repo = Arc::new(MariaDbChartOfAccountRepository::new(pool.clone()));
        let ledger_account_repo = Arc::new(MariaDbLedgerAccountRepository::new(pool.clone()));
        let stmt_repo = Arc::new(MariaDbAccountStmtRepository::new(pool.clone()));
        let line_repo = Arc::new(MariaDbPostingLineRepository::new(pool.clone()));
        let trace_repo = Arc::new(MariaDbPostingTraceRepository::new(pool.clone()));

        let shared_service = SharedService::new(
            coa_repo,
            ledger_repo,
            ledger_account_repo,
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
        let stmt_repo = Arc::new(MariaDbAccountStmtRepository::new(pool.clone()));
        let trace_repo = Arc::new(MariaDbPostingTraceRepository::new(pool.clone()));

        let shared_service = SharedService::new(
            coa_repo,
            ledger_repo,
            ledger_account_repo,
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
            record_user: "test_user".to_string(),
            record_time: chrono::Utc::now(),
            opr_id: format!("test_opr_{}", Uuid::new_v4()),
            opr_time: chrono::Utc::now(),
            opr_type: "test_type".to_string(),
            opr_details: Some("test_details".to_string()),
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
                    details: Some("debit".to_string()),
                    src_account: None,
                    base_line: None,
                    sub_opr_src_id: None,
                    record_time: chrono::Utc::now(),
                    opr_id: "test_opr".to_string(),
                    opr_src: None,
                    pst_time: chrono::Utc::now(),
                    pst_type: postings_api::domain::posting_type::PostingType::BusiTx,
                    pst_status: postings_api::domain::posting_status::PostingStatus::Posted,
                    hash: "hash".to_string(),
                    additional_information: None,
                    discarded_time: None,
                },
                PostingLine {
                    id: Uuid::new_v4(),
                    account: credit_account,
                    debit_amount: BigDecimal::from(0),
                    credit_amount: BigDecimal::from(credit_amount),
                    details: Some("credit".to_string()),
                    src_account: None,
                    base_line: None,
                    sub_opr_src_id: None,
                    record_time: chrono::Utc::now(),
                    opr_id: "test_opr".to_string(),
                    opr_src: None,
                    pst_time: chrono::Utc::now(),
                    pst_type: postings_api::domain::posting_type::PostingType::BusiTx,
                    pst_status: postings_api::domain::posting_status::PostingStatus::Posted,
                    hash: "hash".to_string(),
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
        let expected_details: Vec<Option<String>> = posting_bo.lines.iter().map(|l| l.details.clone()).collect();
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