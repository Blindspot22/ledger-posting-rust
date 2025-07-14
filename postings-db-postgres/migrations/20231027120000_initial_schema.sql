-- Chart of Account
CREATE TABLE chart_of_account (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    created TIMESTAMPTZ NOT NULL,
    user_details VARCHAR(255) NOT NULL,
    short_desc VARCHAR(255),
    long_desc TEXT
);

-- Ledger
CREATE TABLE ledger (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    coa_id UUID NOT NULL REFERENCES chart_of_account(id),
    created TIMESTAMPTZ NOT NULL,
    user_details VARCHAR(255) NOT NULL,
    short_desc VARCHAR(255),
    long_desc TEXT
);

-- BalanceSide Enum
DROP TYPE IF EXISTS balance_side CASCADE;
CREATE TYPE balance_side AS ENUM ('Dr', 'Cr', 'DrCr');

-- AccountCategory Enum
CREATE TYPE account_category AS ENUM ('RE', 'EX', 'AS', 'LI', 'EQ', 'NOOP', 'NORE', 'NOEX');

-- Ledger Account
CREATE TABLE ledger_account (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    ledger_id UUID NOT NULL REFERENCES ledger(id),
    parent_id UUID REFERENCES ledger_account(id),
    coa_id UUID NOT NULL REFERENCES chart_of_account(id),
    balance_side balance_side NOT NULL,
    category account_category NOT NULL,
    created TIMESTAMPTZ NOT NULL,
    user_details VARCHAR(255) NOT NULL,
    short_desc VARCHAR(255),
    long_desc TEXT,
    UNIQUE(ledger_id, name)
);

-- PostingType Enum
CREATE TYPE posting_type AS ENUM ('BUSI_TX', 'ADJ_TX', 'BAL_STMT', 'PNL_STMT', 'BS_STMT', 'LDG_CLSNG');

-- PostingStatus Enum
CREATE TYPE posting_status AS ENUM ('DEFERRED', 'POSTED', 'PROPOSED', 'SIMULATED', 'TAX', 'UNPOSTED', 'CANCELLED', 'OTHER');

-- Posting
CREATE TABLE posting (
    id UUID PRIMARY KEY,
    record_user VARCHAR(255) NOT NULL,
    record_time TIMESTAMPTZ NOT NULL,
    opr_id VARCHAR(255) NOT NULL,
    opr_time TIMESTAMPTZ NOT NULL,
    opr_type VARCHAR(255) NOT NULL,
    opr_details TEXT,
    opr_src VARCHAR(255),
    pst_time TIMESTAMPTZ NOT NULL,
    pst_type posting_type NOT NULL,
    pst_status posting_status NOT NULL,
    ledger_id UUID NOT NULL REFERENCES ledger(id),
    val_time TIMESTAMPTZ,
    discarded_id UUID,
    discarded_time TIMESTAMPTZ,
    discarding_id UUID,
    antecedent_id UUID,
    antecedent_hash VARCHAR(255),
    hash VARCHAR(255),
    hash_alg VARCHAR(255),
    UNIQUE(opr_id, discarding_id)
);

-- Posting Line
CREATE TABLE posting_line (
    id UUID PRIMARY KEY,
    account_id UUID NOT NULL REFERENCES ledger_account(id),
    debit_amount NUMERIC(19, 2) NOT NULL,
    credit_amount NUMERIC(19, 2) NOT NULL,
    details TEXT,
    src_account VARCHAR(255),
    base_line VARCHAR(255),
    sub_opr_src_id VARCHAR(255),
    record_time TIMESTAMPTZ NOT NULL,
    opr_id VARCHAR(255) NOT NULL,
    opr_src VARCHAR(255),
    pst_time TIMESTAMPTZ NOT NULL,
    pst_type posting_type NOT NULL,
    pst_status posting_status NOT NULL,
    hash VARCHAR(255) NOT NULL,
    discarded_time TIMESTAMPTZ
);

-- StmtStatus Enum
CREATE TYPE stmt_status AS ENUM ('SIMULATED', 'CLOSED');

-- Account Statement
CREATE TABLE account_stmt (
    id UUID PRIMARY KEY,
    account_id UUID NOT NULL REFERENCES ledger_account(id),
    youngest_pst_id UUID,
    total_debit NUMERIC(19, 2) NOT NULL,
    total_credit NUMERIC(19, 2) NOT NULL,
    posting_id UUID REFERENCES posting(id),
    pst_time TIMESTAMPTZ NOT NULL,
    stmt_status stmt_status NOT NULL,
    latest_pst_id UUID,
    stmt_seq_nbr INT NOT NULL
);