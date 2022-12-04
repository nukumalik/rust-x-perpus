#![allow(dead_code)]
use crate::traits::builder::Builder;

use super::schema::DistrictSchema;
use chrono::NaiveDateTime;
use uuid::Uuid;

pub struct DistrictBuilder {
    id: String,
    name: Option<String>,
    city_id: Option<String>,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

impl DistrictBuilder {
    pub fn new(name: Option<String>, city_id: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            city_id,
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

    pub fn get_city_id(&self) -> Option<&String> {
        self.city_id.as_ref()
    }

    pub fn set_city_id(&mut self, city_id: Option<String>) {
        self.city_id = city_id;
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

impl From<DistrictSchema> for DistrictBuilder {
    fn from(country: DistrictSchema) -> Self {
        Self {
            id: country.id().clone().to_string(),
            name: Some(country.name().clone().to_string()),
            city_id: Some(country.city_id().clone().to_string()),
            created_at: country.created_at().clone(),
            updated_at: country.updated_at().clone(),
        }
    }
}

impl Default for DistrictBuilder {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: None,
            city_id: None,
            created_at: Some(NaiveDateTime::default()),
            updated_at: Some(NaiveDateTime::default()),
        }
    }
}

impl Builder<DistrictSchema> for DistrictBuilder {
    fn build(&self) -> DistrictSchema {
        DistrictSchema::new(
            self.id.to_string(),
            self.name.as_ref().unwrap().to_string(),
            self.city_id.as_ref().unwrap().to_string(),
            self.created_at.clone(),
            self.updated_at.clone(),
        )
    }
}
