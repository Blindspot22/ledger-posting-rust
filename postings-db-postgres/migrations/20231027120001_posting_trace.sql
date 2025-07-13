-- Posting Trace
CREATE TABLE posting_trace (
    id VARCHAR(255) PRIMARY KEY,
    tgt_pst_id VARCHAR(255) NOT NULL,
    src_pst_time TIMESTAMPTZ NOT NULL,
    src_pst_id VARCHAR(255) NOT NULL,
    src_opr_id VARCHAR(255) NOT NULL,
    account_id VARCHAR(255) NOT NULL REFERENCES ledger_account(id),
    debit_amount NUMERIC(19, 2) NOT NULL,
    credit_amount NUMERIC(19, 2) NOT NULL,
    src_pst_hash VARCHAR(255) NOT NULL,
    UNIQUE(tgt_pst_id, src_pst_id)
);
