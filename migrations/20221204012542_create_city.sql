-- Add migration script here
CREATE TABLE IF NOT EXISTS cities (
    id varchar(100) primary key,
    name varchar(50) not null,
    province_id varchar(100) not null unique,
    created_at datetime default current_timestamp,
    updated_at datetime default current_timestamp
);
-- update_at trigger
CREATE TRIGGER cities_updated_at
AFTER
UPDATE ON cities FOR EACH ROW BEGIN
UPDATE cities
SET updated_at = current_timestamp
WHERE id = old.id;
END;