-- Drop existing named table and recreate with context field and container_type
DROP TABLE IF EXISTS named;

-- Recreate named table with context field and container_type
CREATE TABLE named (
    id CHAR(36) PRIMARY KEY,
    container CHAR(36) NOT NULL,
    context CHAR(36) NOT NULL,
    name VARCHAR(255) NOT NULL,
    language CHAR(2) NOT NULL,
    created DATETIME(6) NOT NULL,
    user_details VARBINARY(34) NOT NULL,
    short_desc VARCHAR(1024),
    long_desc VARCHAR(2048),
    container_type ENUM('ChartOfAccount', 'Ledger', 'LedgerAccount') NOT NULL
);

-- Create indexes for better query performance
CREATE INDEX idx_named_container ON named(container);
CREATE INDEX idx_named_context ON named(context);
CREATE INDEX idx_named_name_type ON named(name, container_type);
CREATE INDEX idx_named_name_type_context ON named(name, container_type, context);

-- Since we're not in production, we don't migrate data
-- In a production environment, you would populate the context field based on business rules:
-- - For ChartOfAccount: context = id (self-referential)
-- - For Ledger: context = coa_id (chart of account)
-- - For LedgerAccount: context = ledger_id (ledger)