-- =============================================================================
-- CONSOLIDATED MARIADB SCHEMA
-- This migration consolidates all previous migrations into a single file
-- that matches the current model definitions exactly
-- =============================================================================

-- Drop all existing objects to ensure clean state
DROP TABLE IF EXISTS posting_trace;
DROP TABLE IF EXISTS account_stmt; 
DROP TABLE IF EXISTS posting_line;
DROP TABLE IF EXISTS posting;
DROP TABLE IF EXISTS named;
DROP TABLE IF EXISTS ledger_account;
DROP TABLE IF EXISTS ledger;
DROP TABLE IF EXISTS chart_of_account;

-- =============================================================================
-- CORE TABLES
-- =============================================================================

-- Chart of Account (simplified after named entity decoupling)
CREATE TABLE chart_of_account (
    id CHAR(36) PRIMARY KEY
) ENGINE=InnoDB;

-- Ledger (simplified after named entity decoupling)
CREATE TABLE ledger (
    id CHAR(36) PRIMARY KEY,
    coa_id CHAR(36) NOT NULL,
    FOREIGN KEY (coa_id) REFERENCES chart_of_account(id)
) ENGINE=InnoDB;

-- Ledger Account (simplified after named entity decoupling)
CREATE TABLE ledger_account (
    id CHAR(36) PRIMARY KEY,
    ledger_id CHAR(36) NOT NULL,
    parent_id CHAR(36),
    coa_id CHAR(36) NOT NULL,
    balance_side ENUM('Dr', 'Cr', 'DrCr') NOT NULL,
    category ENUM('RE', 'EX', 'AS', 'LI', 'EQ', 'NOOP', 'NORE', 'NOEX') NOT NULL,
    FOREIGN KEY (ledger_id) REFERENCES ledger(id),
    FOREIGN KEY (parent_id) REFERENCES ledger_account(id),
    FOREIGN KEY (coa_id) REFERENCES chart_of_account(id),
    UNIQUE KEY unique_ledger_account (ledger_id, id)
) ENGINE=InnoDB;

-- Named entity table (contains all naming and descriptive information)
CREATE TABLE named (
    id VARCHAR(255) PRIMARY KEY,
    container VARCHAR(255) NOT NULL, -- References the entity this name belongs to
    context VARCHAR(255) NOT NULL,   -- References the broader context (COA for Ledger, Ledger for LedgerAccount)
    name VARCHAR(255) NOT NULL,
    language CHAR(2) NOT NULL,
    created TIMESTAMP NOT NULL,
    user_details BLOB NOT NULL, -- Binary hash data
    short_desc VARCHAR(1024),
    long_desc VARCHAR(2048),
    container_type ENUM('ChartOfAccount', 'Ledger', 'LedgerAccount') NOT NULL
) ENGINE=InnoDB;

-- =============================================================================
-- POSTING TABLES  
-- =============================================================================

-- Posting (with all binary hash fields as BLOB)
CREATE TABLE posting (
    id CHAR(36) PRIMARY KEY,
    record_user BLOB NOT NULL,        -- Binary hash
    record_time TIMESTAMP NOT NULL,
    opr_id BLOB NOT NULL,             -- Binary hash
    opr_time TIMESTAMP NOT NULL,
    opr_type BLOB NOT NULL,           -- Binary hash
    opr_details BLOB,                 -- Binary hash (optional)
    opr_src BLOB,                     -- Binary hash (optional)
    pst_time TIMESTAMP NOT NULL,
    pst_type ENUM('BUSI_TX', 'ADJ_TX', 'BAL_STMT', 'PNL_STMT', 'BS_STMT', 'LDG_CLSNG') NOT NULL,
    pst_status ENUM('DEFERRED', 'POSTED', 'PROPOSED', 'SIMULATED', 'TAX', 'UNPOSTED', 'CANCELLED', 'OTHER') NOT NULL,
    ledger_id CHAR(36) NOT NULL,
    val_time TIMESTAMP NULL,
    discarded_id CHAR(36),
    discarded_time TIMESTAMP NULL,
    discarding_id CHAR(36),
    antecedent_id CHAR(36),
    antecedent_hash BLOB,             -- Binary hash (optional)
    hash BLOB,                        -- Binary hash (optional)
    FOREIGN KEY (ledger_id) REFERENCES ledger(id),
    UNIQUE KEY unique_opr_discarding (opr_id(34), discarding_id)
) ENGINE=InnoDB;

-- Posting Line (with all binary hash fields as BLOB)
CREATE TABLE posting_line (
    id CHAR(36) PRIMARY KEY,
    account_id CHAR(36) NOT NULL,
    debit_amount DECIMAL(19, 2) NOT NULL,
    credit_amount DECIMAL(19, 2) NOT NULL,
    details BLOB,                     -- Binary hash (optional)
    src_account BLOB,                 -- Binary hash (optional)
    base_line CHAR(36),               -- UUID reference
    sub_opr_src_id BLOB,              -- Binary hash (optional)
    record_time TIMESTAMP NOT NULL,
    opr_id BLOB NOT NULL,             -- Binary hash
    opr_src BLOB,                     -- Binary hash (optional)
    pst_time TIMESTAMP NOT NULL,
    pst_type ENUM('BUSI_TX', 'ADJ_TX', 'BAL_STMT', 'PNL_STMT', 'BS_STMT', 'LDG_CLSNG') NOT NULL,
    pst_status ENUM('DEFERRED', 'POSTED', 'PROPOSED', 'SIMULATED', 'TAX', 'UNPOSTED', 'CANCELLED', 'OTHER') NOT NULL,
    hash BLOB,                        -- Binary hash (optional)
    discarded_time TIMESTAMP NULL,
    FOREIGN KEY (account_id) REFERENCES ledger_account(id)
) ENGINE=InnoDB;

-- =============================================================================
-- STATEMENT TABLES
-- =============================================================================

-- Account Statement
CREATE TABLE account_stmt (
    id CHAR(36) PRIMARY KEY,
    account_id CHAR(36) NOT NULL,
    youngest_pst_id CHAR(36),
    total_debit DECIMAL(19, 2) NOT NULL,
    total_credit DECIMAL(19, 2) NOT NULL,
    posting_id CHAR(36),
    pst_time TIMESTAMP NOT NULL,
    stmt_status ENUM('SIMULATED', 'CLOSED') NOT NULL,
    latest_pst_id CHAR(36),
    stmt_seq_nbr INT NOT NULL,
    FOREIGN KEY (account_id) REFERENCES ledger_account(id),
    FOREIGN KEY (posting_id) REFERENCES posting(id)
) ENGINE=InnoDB;

-- Posting Trace
CREATE TABLE posting_trace (
    id CHAR(36) PRIMARY KEY,
    tgt_pst_id CHAR(36) NOT NULL,
    src_pst_time TIMESTAMP NOT NULL,
    src_pst_id CHAR(36) NOT NULL,
    src_opr_id BLOB NOT NULL,         -- Binary hash
    account_id CHAR(36) NOT NULL,
    debit_amount DECIMAL(19, 2) NOT NULL,
    credit_amount DECIMAL(19, 2) NOT NULL,
    src_pst_hash BLOB,                -- Binary hash (optional)
    FOREIGN KEY (account_id) REFERENCES ledger_account(id),
    UNIQUE KEY unique_trace (tgt_pst_id, src_pst_id)
) ENGINE=InnoDB;

-- =============================================================================
-- INDEXES FOR PERFORMANCE
-- =============================================================================

-- Named entity indexes
CREATE INDEX idx_named_container ON named(container);
CREATE INDEX idx_named_context ON named(context);
CREATE INDEX idx_named_name_type ON named(name, container_type);
CREATE INDEX idx_named_name_type_context ON named(name, container_type, context);

-- Posting indexes
CREATE INDEX idx_posting_ledger_id ON posting(ledger_id);
CREATE INDEX idx_posting_opr_id ON posting(opr_id(34));
CREATE INDEX idx_posting_pst_time ON posting(pst_time);
CREATE INDEX idx_posting_discarding_id ON posting(discarding_id);

-- Posting line indexes  
CREATE INDEX idx_posting_line_account_id ON posting_line(account_id);
CREATE INDEX idx_posting_line_opr_id ON posting_line(opr_id(34));
CREATE INDEX idx_posting_line_pst_time ON posting_line(pst_time);
CREATE INDEX idx_posting_line_base_line ON posting_line(base_line);

-- Ledger account indexes
CREATE INDEX idx_ledger_account_ledger_id ON ledger_account(ledger_id);
CREATE INDEX idx_ledger_account_parent_id ON ledger_account(parent_id);
CREATE INDEX idx_ledger_account_coa_id ON ledger_account(coa_id);

-- Account statement indexes
CREATE INDEX idx_account_stmt_account_id ON account_stmt(account_id);
CREATE INDEX idx_account_stmt_pst_time ON account_stmt(pst_time);

-- Posting trace indexes
CREATE INDEX idx_posting_trace_tgt_pst_id ON posting_trace(tgt_pst_id);
CREATE INDEX idx_posting_trace_src_pst_id ON posting_trace(src_pst_id);
CREATE INDEX idx_posting_trace_account_id ON posting_trace(account_id);