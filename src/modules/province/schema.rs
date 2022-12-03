#![allow(dead_code)]
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct ProvinceSchema {
    id: String,
    name: String,
    country_id: String,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

impl ProvinceSchema {
    pub fn new(
        id: String,
        name: String,
        country_id: String,
        created_at: Option<NaiveDateTime>,
        updated_at: Option<NaiveDateTime>,
    ) -> Self {
        Self {
            id,
            name,
            country_id,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn country_id(&self) -> &String {
        &self.country_id
    }

    pub fn created_at(&self) -> &Option<NaiveDateTime> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &Option<NaiveDateTime> {
        &self.updated_at
    }
}
