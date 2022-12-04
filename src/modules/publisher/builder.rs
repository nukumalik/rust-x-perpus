#![allow(dead_code)]
use super::schema::PublisherSchema;
use crate::traits::builder::Builder;
use chrono::NaiveDateTime;
use uuid::Uuid;

pub struct PublisherBuilder {
    id: String,
    name: Option<String>,
    country_id: Option<String>,
    province_id: Option<String>,
    city_id: Option<String>,
    district_id: Option<String>,
    street: Option<String>,
    zip_code: Option<String>,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

impl PublisherBuilder {
    pub fn new(
        name: Option<String>,
        country_id: Option<String>,
        province_id: Option<String>,
        city_id: Option<String>,
        district_id: Option<String>,
        street: Option<String>,
        zip_code: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            country_id,
            province_id,
            city_id,
            district_id,
            street,
            zip_code,
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

    pub fn get_province_id(&self) -> Option<&String> {
        self.province_id.as_ref()
    }

    pub fn set_province_id(&mut self, province_id: Option<String>) {
        self.province_id = province_id;
    }

    pub fn get_city_id(&self) -> Option<&String> {
        self.city_id.as_ref()
    }

    pub fn set_city_id(&mut self, city_id: Option<String>) {
        self.city_id = city_id;
    }

    pub fn get_district_id(&self) -> Option<&String> {
        self.district_id.as_ref()
    }

    pub fn set_district_id(&mut self, district_id: Option<String>) {
        self.district_id = district_id;
    }

    pub fn get_street(&self) -> Option<&String> {
        self.street.as_ref()
    }

    pub fn set_street(&mut self, street: Option<String>) {
        self.street = street;
    }

    pub fn get_zip_code(&self) -> Option<&String> {
        self.zip_code.as_ref()
    }

    pub fn set_zip_code(&mut self, zip_code: Option<String>) {
        self.zip_code = zip_code;
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

impl From<PublisherSchema> for PublisherBuilder {
    fn from(publisher: PublisherSchema) -> Self {
        Self {
            id: publisher.id().clone().to_string(),
            name: Some(publisher.name().clone().to_string()),
            country_id: Some(publisher.country_id().clone().to_string()),
            province_id: Some(publisher.province_id().clone().to_string()),
            city_id: Some(publisher.city_id().clone().to_string()),
            district_id: Some(publisher.district_id().clone().to_string()),
            street: Some(publisher.street().clone().to_string()),
            zip_code: Some(publisher.zip_code().clone().to_string()),
            created_at: publisher.created_at().clone(),
            updated_at: publisher.updated_at().clone(),
        }
    }
}

impl Default for PublisherBuilder {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: None,
            country_id: None,
            province_id: None,
            city_id: None,
            district_id: None,
            street: None,
            zip_code: None,
            created_at: Some(NaiveDateTime::default()),
            updated_at: Some(NaiveDateTime::default()),
        }
    }
}

impl Builder<PublisherSchema> for PublisherBuilder {
    fn build(&self) -> PublisherSchema {
        PublisherSchema::new(
            self.id.to_string(),
            self.name.as_ref().unwrap().to_string(),
            self.country_id.as_ref().unwrap().to_string(),
            self.province_id.as_ref().unwrap().to_string(),
            self.city_id.as_ref().unwrap().to_string(),
            self.district_id.as_ref().unwrap().to_string(),
            self.street.as_ref().unwrap().to_string(),
            self.zip_code.as_ref().unwrap().to_string(),
            self.created_at.clone(),
            self.updated_at.clone(),
        )
    }
}
