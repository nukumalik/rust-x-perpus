#![allow(dead_code)]
use crate::traits::builder::Builder;

use super::schema::ProvinceSchema;
use chrono::NaiveDateTime;
use uuid::Uuid;

pub struct ProvinceBuilder {
    id: String,
    name: Option<String>,
    country_id: Option<String>,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

impl ProvinceBuilder {
    pub fn new(name: Option<String>, country_id: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            country_id,
            created_at: Some(NaiveDateTime::default()),
            updated_at: Some(NaiveDateTime::default()),
        }
    }

    pub fn get_id(&self) -> &str {
        self.id.as_ref()
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name;
    }

    pub fn get_country_id(&self) -> Option<&String> {
        self.country_id.as_ref()
    }

    pub fn set_country_id(&mut self, country_id: Option<String>) {
        self.country_id = country_id;
    }

    pub fn get_created_at(&self) -> Option<NaiveDateTime> {
        self.created_at
    }

    pub fn set_created_at(&mut self, created_at: Option<NaiveDateTime>) {
        self.created_at = created_at;
    }

    pub fn get_updated_at(&self) -> Option<NaiveDateTime> {
        self.updated_at
    }

    pub fn set_updated_at(&mut self, updated_at: Option<NaiveDateTime>) {
        self.updated_at = updated_at;
    }
}

impl From<ProvinceSchema> for ProvinceBuilder {
    fn from(country: ProvinceSchema) -> Self {
        Self {
            id: country.id().clone().to_string(),
            name: Some(country.name().clone().to_string()),
            country_id: Some(country.country_id().clone().to_string()),
            created_at: country.created_at().clone(),
            updated_at: country.updated_at().clone(),
        }
    }
}

impl Default for ProvinceBuilder {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: None,
            country_id: None,
            created_at: Some(NaiveDateTime::default()),
            updated_at: Some(NaiveDateTime::default()),
        }
    }
}

impl Builder<ProvinceSchema> for ProvinceBuilder {
    fn build(&self) -> ProvinceSchema {
        ProvinceSchema::new(
            self.id.to_string(),
            self.name.as_ref().unwrap().to_string(),
            self.country_id.as_ref().unwrap().to_string(),
            self.created_at.clone(),
            self.updated_at.clone(),
        )
    }
}
