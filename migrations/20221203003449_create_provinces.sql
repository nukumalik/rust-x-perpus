-- Add migration script here
CREATE TABLE IF NOT EXISTS provinces (
    id varchar(100) primary key,
    name varchar(50) not null,
    country_id varchar(20) not null unique,
    created_at datetime default current_timestamp,
    updated_at datetime default current_timestamp
);
-- update_at trigger
CREATE TRIGGER provinces_updated_at
AFTER
UPDATE ON provinces FOR EACH ROW BEGIN
UPDATE provinces
SET updated_at = current_timestamp
WHERE id = old.id;
END;