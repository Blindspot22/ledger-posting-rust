-- Create named table
CREATE TABLE named (
    id CHAR(36) PRIMARY KEY,
    container CHAR(36) NOT NULL,
    name VARCHAR(255) NOT NULL,
    language CHAR(2) NOT NULL,
    created DATETIME(6) NOT NULL,
    user_details VARBINARY(32) NOT NULL,
    short_desc VARCHAR(1024),
    long_desc VARCHAR(2048)
);

-- Migrate data from chart_of_account to named
INSERT INTO named (id, container, name, language, created, user_details, short_desc, long_desc)
SELECT id, id, name, 'en', created, SHA2(user_details, 256), short_desc, long_desc FROM chart_of_account;

-- Migrate data from ledger to named
INSERT INTO named (id, container, name, language, created, user_details, short_desc, long_desc)
SELECT id, id, name, 'en', created, SHA2(user_details, 256), short_desc, long_desc FROM ledger;

-- Migrate data from ledger_account to named
INSERT INTO named (id, container, name, language, created, user_details, short_desc, long_desc)
SELECT id, id, name, 'en', created, SHA2(user_details, 256), short_desc, long_desc FROM ledger_account;

-- Alter chart_of_account table
ALTER TABLE chart_of_account
DROP COLUMN name,
DROP COLUMN created,
DROP COLUMN user_details,
DROP COLUMN short_desc,
DROP COLUMN long_desc;

-- Alter ledger table
ALTER TABLE ledger
DROP COLUMN name,
DROP COLUMN created,
DROP COLUMN user_details,
DROP COLUMN short_desc,
DROP COLUMN long_desc;

-- Alter ledger_account table
ALTER TABLE ledger_account
DROP COLUMN name,
DROP COLUMN created,
DROP COLUMN user_details,
DROP COLUMN short_desc,
DROP COLUMN long_desc;
