#![allow(dead_code)]
use crate::traits::builder::Builder;

use super::schema::CountrySchema;
use chrono::NaiveDateTime;
use uuid::Uuid;

pub struct CountryBuilder {
    id: String,
    name: Option<String>,
    code: Option<String>,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

impl CountryBuilder {
    pub fn new(name: Option<String>, code: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            code,
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

    pub fn get_code(&self) -> Option<&String> {
        self.code.as_ref()
    }

    pub fn set_code(&mut self, code: Option<String>) {
        self.code = code;
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

impl From<CountrySchema> for CountryBuilder {
    fn from(country: CountrySchema) -> Self {
        Self {
            id: country.id().clone().to_string(),
            name: Some(country.name().clone().to_string()),
            code: Some(country.code().clone().to_string()),
            created_at: country.created_at().clone(),
            updated_at: country.updated_at().clone(),
        }
    }
}

impl Default for CountryBuilder {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: None,
            code: None,
            created_at: Some(NaiveDateTime::default()),
            updated_at: Some(NaiveDateTime::default()),
        }
    }
}

impl Builder<CountrySchema> for CountryBuilder {
    fn build(&self) -> CountrySchema {
        CountrySchema::new(
            self.id.to_string(),
            self.name.as_ref().unwrap().to_string(),
            self.code.as_ref().unwrap().to_string(),
            self.created_at.clone(),
            self.updated_at.clone(),
        )
    }
}
