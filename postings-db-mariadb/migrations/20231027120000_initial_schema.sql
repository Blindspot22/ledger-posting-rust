
-- Chart of Account
CREATE TABLE chart_of_account (
    id CHAR(36) PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    created DATETIME(6) NOT NULL,
    user_details VARCHAR(255) NOT NULL,
    short_desc VARCHAR(255),
    long_desc TEXT
);

-- Ledger
CREATE TABLE ledger (
    id CHAR(36) PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    coa_id CHAR(36) NOT NULL,
    created DATETIME(6) NOT NULL,
    user_details VARCHAR(255) NOT NULL,
    short_desc VARCHAR(255),
    long_desc TEXT,
    FOREIGN KEY (coa_id) REFERENCES chart_of_account(id)
);

-- Ledger Account
CREATE TABLE ledger_account (
    id CHAR(36) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    ledger_id CHAR(36) NOT NULL,
    parent_id CHAR(36),
    coa_id CHAR(36) NOT NULL,
    balance_side ENUM('Dr', 'Cr', 'DrCr') NOT NULL,
    category ENUM('RE', 'EX', 'AS', 'LI', 'EQ', 'NOOP', 'NORE', 'NOEX') NOT NULL,
    created DATETIME(6) NOT NULL,
    user_details VARCHAR(255) NOT NULL,
    short_desc VARCHAR(255),
    long_desc TEXT,
    UNIQUE(ledger_id, name),
    FOREIGN KEY (ledger_id) REFERENCES ledger(id),
    FOREIGN KEY (parent_id) REFERENCES ledger_account(id),
    FOREIGN KEY (coa_id) REFERENCES chart_of_account(id)
);

-- Posting
CREATE TABLE posting (
    id CHAR(36) PRIMARY KEY,
    record_user VARCHAR(255) NOT NULL,
    record_time DATETIME(6) NOT NULL,
    opr_id VARCHAR(255) NOT NULL,
    opr_time DATETIME(6) NOT NULL,
    opr_type VARCHAR(255) NOT NULL,
    opr_details TEXT,
    opr_src VARCHAR(255),
    pst_time DATETIME(6) NOT NULL,
    pst_type ENUM('BusiTx', 'AdjTx', 'BalStmt', 'PnlStmt', 'BsStmt', 'LdgClsng') NOT NULL,
    pst_status ENUM('Deferred', 'Posted', 'Proposed', 'Simulated', 'Tax', 'Unposted', 'Cancelled', 'Other') NOT NULL,
    ledger_id CHAR(36) NOT NULL,
    val_time DATETIME(6),
    discarded_id CHAR(36),
    discarded_time DATETIME(6),
    discarding_id CHAR(36),
    antecedent_id CHAR(36),
    antecedent_hash VARCHAR(255),
    hash VARCHAR(255),
    hash_alg VARCHAR(255),
    UNIQUE(opr_id, discarding_id),
    FOREIGN KEY (ledger_id) REFERENCES ledger(id)
);

-- Posting Line
CREATE TABLE posting_line (
    id CHAR(36) PRIMARY KEY,
    account_id CHAR(36) NOT NULL,
    debit_amount DECIMAL(19, 2) NOT NULL,
    credit_amount DECIMAL(19, 2) NOT NULL,
    details TEXT,
    src_account VARCHAR(255),
    base_line VARCHAR(255),
    sub_opr_src_id VARCHAR(255),
    record_time DATETIME(6) NOT NULL,
    opr_id VARCHAR(255) NOT NULL,
    opr_src VARCHAR(255),
    pst_time DATETIME(6) NOT NULL,
    pst_type ENUM('BusiTx', 'AdjTx', 'BalStmt', 'PnlStmt', 'BsStmt', 'LdgClsng') NOT NULL,
    pst_status ENUM('Deferred', 'Posted', 'Proposed', 'Simulated', 'Tax', 'Unposted', 'Cancelled', 'Other') NOT NULL,
    hash VARCHAR(255) NOT NULL,
    discarded_time DATETIME(6),
    FOREIGN KEY (account_id) REFERENCES ledger_account(id)
);

-- Account Statement
CREATE TABLE account_stmt (
    id CHAR(36) PRIMARY KEY,
    account_id CHAR(36) NOT NULL,
    youngest_pst_id CHAR(36),
    total_debit DECIMAL(19, 2) NOT NULL,
    total_credit DECIMAL(19, 2) NOT NULL,
    posting_id CHAR(36),
    pst_time DATETIME(6) NOT NULL,
    stmt_status ENUM('Simulated', 'Closed') NOT NULL,
    latest_pst_id CHAR(36),
    stmt_seq_nbr INT NOT NULL,
    FOREIGN KEY (account_id) REFERENCES ledger_account(id),
    FOREIGN KEY (posting_id) REFERENCES posting(id)
);

-- Posting Trace
CREATE TABLE posting_trace (
    id CHAR(36) PRIMARY KEY,
    tgt_pst_id CHAR(36) NOT NULL,
    src_pst_time DATETIME(6) NOT NULL,
    src_pst_id CHAR(36) NOT NULL,
    src_opr_id VARCHAR(255) NOT NULL,
    account_id CHAR(36) NOT NULL,
    debit_amount DECIMAL(19, 2) NOT NULL,
    credit_amount DECIMAL(19, 2) NOT NULL,
    src_pst_hash VARCHAR(255) NOT NULL,
    UNIQUE(tgt_pst_id, src_pst_id),
    FOREIGN KEY (account_id) REFERENCES ledger_account(id)
);
