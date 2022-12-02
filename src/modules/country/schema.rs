#![allow(dead_code)]
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct CountrySchema {
    id: String,
    name: String,
    code: String,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

impl CountrySchema {
    pub fn new(
        id: String,
        name: String,
        code: String,
        created_at: Option<NaiveDateTime>,
        updated_at: Option<NaiveDateTime>,
    ) -> Self {
        Self {
            id,
            name,
            code,
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

    pub fn code(&self) -> &String {
        &self.code
    }

    pub fn created_at(&self) -> &Option<NaiveDateTime> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &Option<NaiveDateTime> {
        &self.updated_at
    }
}
