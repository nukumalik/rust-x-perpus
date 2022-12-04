-- Add migration script here
CREATE TABLE IF NOT EXISTS publishers (
    id varchar(100) primary key,
    name varchar(50) not null,
    country_id varchar(100) not null,
    province_id varchar(100) not null,
    city_id varchar(100) not null,
    district_id varchar(100) not null,
    street varchar(100) not null,
    zip_code varchar(100) not null,
    created_at datetime default current_timestamp,
    updated_at datetime default current_timestamp
);
-- update_at trigger
CREATE TRIGGER publishers_updated_at
AFTER
UPDATE ON publishers FOR EACH ROW BEGIN
UPDATE publishers
SET updated_at = current_timestamp
WHERE id = old.id;
END;