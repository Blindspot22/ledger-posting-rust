-- =============================================================================
-- CONSOLIDATED POSTGRESQL SCHEMA
-- This migration consolidates all previous migrations into a single file
-- that matches the current model definitions exactly
-- =============================================================================

-- Drop all existing objects to ensure clean state
DROP TABLE IF EXISTS posting_trace CASCADE;
DROP TABLE IF EXISTS account_stmt CASCADE; 
DROP TABLE IF EXISTS posting_line CASCADE;
DROP TABLE IF EXISTS posting CASCADE;
DROP TABLE IF EXISTS named CASCADE;
DROP TABLE IF EXISTS ledger_account CASCADE;
DROP TABLE IF EXISTS ledger CASCADE;
DROP TABLE IF EXISTS chart_of_account CASCADE;

-- Drop existing types
DROP TYPE IF EXISTS stmt_status CASCADE;
DROP TYPE IF EXISTS container_type CASCADE;
DROP TYPE IF EXISTS posting_status CASCADE;
DROP TYPE IF EXISTS posting_type CASCADE;
DROP TYPE IF EXISTS account_category CASCADE;
DROP TYPE IF EXISTS balance_side CASCADE;

-- =============================================================================
-- ENUMS
-- =============================================================================

-- Balance Side Enum
CREATE TYPE balance_side AS ENUM ('Dr', 'Cr', 'DrCr');

-- Account Category Enum  
CREATE TYPE account_category AS ENUM ('RE', 'EX', 'AS', 'LI', 'EQ', 'NOOP', 'NORE', 'NOEX');

-- Posting Type Enum
CREATE TYPE posting_type AS ENUM ('BUSI_TX', 'ADJ_TX', 'BAL_STMT', 'PNL_STMT', 'BS_STMT', 'LDG_CLSNG');

-- Posting Status Enum
CREATE TYPE posting_status AS ENUM ('DEFERRED', 'POSTED', 'PROPOSED', 'SIMULATED', 'TAX', 'UNPOSTED', 'CANCELLED', 'OTHER');

-- Container Type Enum (for named entities)
CREATE TYPE container_type AS ENUM ('ChartOfAccount', 'Ledger', 'LedgerAccount');

-- Statement Status Enum
CREATE TYPE stmt_status AS ENUM ('SIMULATED', 'CLOSED');

-- =============================================================================
-- CORE TABLES
-- =============================================================================

-- Chart of Account (simplified after named entity decoupling)
CREATE TABLE chart_of_account (
    id UUID PRIMARY KEY
);

-- Ledger (simplified after named entity decoupling)
CREATE TABLE ledger (
    id UUID PRIMARY KEY,
    coa_id UUID NOT NULL REFERENCES chart_of_account(id)
);

-- Ledger Account (simplified after named entity decoupling)
CREATE TABLE ledger_account (
    id UUID PRIMARY KEY,
    ledger_id UUID NOT NULL REFERENCES ledger(id),
    parent_id UUID REFERENCES ledger_account(id),
    coa_id UUID NOT NULL REFERENCES chart_of_account(id),
    balance_side balance_side NOT NULL,
    category account_category NOT NULL,
    UNIQUE(ledger_id, id) -- Ensures uniqueness within ledger context
);

-- Named entity table (contains all naming and descriptive information)
CREATE TABLE named (
    id UUID PRIMARY KEY,
    container UUID NOT NULL, -- References the entity this name belongs to
    context UUID NOT NULL,   -- References the broader context (COA for Ledger, Ledger for LedgerAccount)
    name VARCHAR(255) NOT NULL,
    language CHAR(2) NOT NULL,
    created TIMESTAMPTZ NOT NULL,
    user_details BYTEA NOT NULL, -- 34-byte hash
    short_desc VARCHAR(1024),
    long_desc VARCHAR(2048),
    container_type container_type NOT NULL
);

-- =============================================================================
-- POSTING TABLES  
-- =============================================================================

-- Posting (with all binary hash fields)
CREATE TABLE posting (
    id UUID PRIMARY KEY,
    record_user BYTEA NOT NULL,        -- 34-byte hash
    record_time TIMESTAMPTZ NOT NULL,
    opr_id BYTEA NOT NULL,             -- 34-byte hash
    opr_time TIMESTAMPTZ NOT NULL,
    opr_type BYTEA NOT NULL,           -- 34-byte hash
    opr_details BYTEA,                 -- 34-byte hash (optional)
    opr_src BYTEA,                     -- 34-byte hash (optional)
    pst_time TIMESTAMPTZ NOT NULL,
    pst_type posting_type NOT NULL,
    pst_status posting_status NOT NULL,
    ledger_id UUID NOT NULL REFERENCES ledger(id),
    val_time TIMESTAMPTZ,
    discarded_id UUID,
    discarded_time TIMESTAMPTZ,
    discarding_id UUID,
    antecedent_id UUID,
    antecedent_hash BYTEA,             -- 34-byte hash (optional)
    hash BYTEA,                        -- 34-byte hash (optional)
    UNIQUE(opr_id, discarding_id)
);

-- Posting Line (with all binary hash fields)
CREATE TABLE posting_line (
    id UUID PRIMARY KEY,
    account_id UUID NOT NULL REFERENCES ledger_account(id),
    debit_amount NUMERIC(19, 2) NOT NULL,
    credit_amount NUMERIC(19, 2) NOT NULL,
    details BYTEA,                     -- 34-byte hash (optional)
    src_account BYTEA,                 -- 34-byte hash (optional)
    base_line UUID,                    -- Changed to UUID as per model
    sub_opr_src_id BYTEA,              -- 34-byte hash (optional)
    record_time TIMESTAMPTZ NOT NULL,
    opr_id BYTEA NOT NULL,             -- 34-byte hash
    opr_src BYTEA,                     -- 34-byte hash (optional)
    pst_time TIMESTAMPTZ NOT NULL,
    pst_type posting_type NOT NULL,
    pst_status posting_status NOT NULL,
    hash BYTEA,                        -- 34-byte hash (optional)
    discarded_time TIMESTAMPTZ
);

-- =============================================================================
-- STATEMENT TABLES
-- =============================================================================

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

-- Posting Trace
CREATE TABLE posting_trace (
    id UUID PRIMARY KEY,
    tgt_pst_id UUID NOT NULL,
    src_pst_time TIMESTAMPTZ NOT NULL,
    src_pst_id UUID NOT NULL,
    src_opr_id BYTEA NOT NULL,         -- 34-byte hash
    account_id UUID NOT NULL REFERENCES ledger_account(id),
    debit_amount NUMERIC(19, 2) NOT NULL,
    credit_amount NUMERIC(19, 2) NOT NULL,
    src_pst_hash BYTEA NOT NULL,       -- 34-byte hash
    UNIQUE(tgt_pst_id, src_pst_id)
);

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
CREATE INDEX idx_posting_opr_id ON posting(opr_id);
CREATE INDEX idx_posting_pst_time ON posting(pst_time);
CREATE INDEX idx_posting_discarding_id ON posting(discarding_id);

-- Posting line indexes  
CREATE INDEX idx_posting_line_account_id ON posting_line(account_id);
CREATE INDEX idx_posting_line_opr_id ON posting_line(opr_id);
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

-- =============================================================================
-- COMMENTS FOR DOCUMENTATION
-- =============================================================================

COMMENT ON TABLE chart_of_account IS 'Chart of Accounts - contains only the ID, with all descriptive data in named table';
COMMENT ON TABLE ledger IS 'Ledgers within a chart of accounts - contains only structural data';
COMMENT ON TABLE ledger_account IS 'Accounts within ledgers - contains only structural and categorization data';
COMMENT ON TABLE named IS 'Named entities - contains all naming, descriptive, and metadata for chart of accounts, ledgers, and accounts';
COMMENT ON TABLE posting IS 'Journal postings - financial transactions with full audit trail';
COMMENT ON TABLE posting_line IS 'Individual lines within postings - the double-entry bookkeeping entries';

COMMENT ON COLUMN named.context IS 'Broader organizational context: COA ID for ledgers, Ledger ID for accounts, self-reference for COAs';
COMMENT ON COLUMN named.container IS 'Direct container: the entity this name record describes';
COMMENT ON COLUMN named.user_details IS '34-byte hash of user details for security';

COMMENT ON COLUMN posting.record_user IS '34-byte hash of the user who recorded this posting';
COMMENT ON COLUMN posting.opr_id IS '34-byte hash of operation identifier';
COMMENT ON COLUMN posting.opr_type IS '34-byte hash of operation type';
COMMENT ON COLUMN posting.hash IS '34-byte multihash for integrity chain';
COMMENT ON COLUMN posting.antecedent_hash IS '34-byte multihash of previous posting in chain';