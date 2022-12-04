-- Add migration script here
CREATE TABLE IF NOT EXISTS districts (
    id varchar(100) primary key,
    name varchar(50) not null,
    city_id varchar(100) not null unique,
    created_at datetime default current_timestamp,
    updated_at datetime default current_timestamp
);
-- update_at trigger
CREATE TRIGGER districts_updated_at
AFTER
UPDATE ON districts FOR EACH ROW BEGIN
UPDATE districts
SET updated_at = current_timestamp
WHERE id = old.id;
END;