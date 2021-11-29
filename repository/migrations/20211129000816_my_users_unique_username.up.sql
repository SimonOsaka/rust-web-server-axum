-- Add up migration script here
ALTER TABLE my_users
ADD CONSTRAINT uk_username UNIQUE(username);