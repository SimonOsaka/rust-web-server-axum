-- Add down migration script here
ALTER TABLE my_users
DROP CONSTRAINT uk_username;