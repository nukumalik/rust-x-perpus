#![allow(dead_code)]
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct PublisherSchema {
    id: String,
    name: String,
    country_id: String,
    province_id: String,
    city_id: String,
    district_id: String,
    street: String,
    zip_code: String,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

impl PublisherSchema {
    pub fn new(
        id: String,
        name: String,
        country_id: String,
        province_id: String,
        city_id: String,
        district_id: String,
        street: String,
        zip_code: String,
        created_at: Option<NaiveDateTime>,
        updated_at: Option<NaiveDateTime>,
    ) -> Self {
        Self {
            id,
            name,
            country_id,
            province_id,
            city_id,
            district_id,
            street,
            zip_code,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> &str {
        self.id.as_ref()
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn country_id(&self) -> &str {
        self.country_id.as_ref()
    }

    pub fn province_id(&self) -> &str {
        self.province_id.as_ref()
    }

    pub fn city_id(&self) -> &str {
        self.city_id.as_ref()
    }

    pub fn district_id(&self) -> &str {
        self.district_id.as_ref()
    }

    pub fn street(&self) -> &str {
        self.street.as_ref()
    }

    pub fn zip_code(&self) -> &str {
        self.zip_code.as_ref()
    }

    pub fn created_at(&self) -> Option<NaiveDateTime> {
        self.created_at
    }

    pub fn updated_at(&self) -> Option<NaiveDateTime> {
        self.updated_at
    }
}
