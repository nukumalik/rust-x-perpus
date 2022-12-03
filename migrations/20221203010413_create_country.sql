-- Add migration script here
CREATE TABLE IF NOT EXISTS countries (
    id varchar(100) primary key,
    name varchar(50) not null,
    code varchar(10) not null unique,
    created_at datetime default current_timestamp,
    updated_at datetime default current_timestamp
);
-- update_at trigger
CREATE TRIGGER countries_updated_at
AFTER
UPDATE ON countries FOR EACH ROW BEGIN
UPDATE countries
SET updated_at = current_timestamp
WHERE id = old.id;
END;