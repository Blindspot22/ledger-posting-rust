-- Grant database creation and management privileges to the test user
-- This is needed for sqlx testing framework which creates temporary test databases

GRANT ALL PRIVILEGES ON *.* TO 'user'@'%' WITH GRANT OPTION;
FLUSH PRIVILEGES;

-- Verify the grants (optional, for debugging)
SHOW GRANTS FOR 'user'@'%';