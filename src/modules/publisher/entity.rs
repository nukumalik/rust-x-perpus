#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePublisherPayload {
    name: String,
    country_id: String,
    province_id: String,
    city_id: String,
    district_id: String,
    street: String,
    zip_code: String,
}

impl CreatePublisherPayload {
    pub fn get_name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn get_country_id(&self) -> &str {
        self.country_id.as_ref()
    }

    pub fn get_province_id(&self) -> &str {
        self.province_id.as_ref()
    }

    pub fn get_city_id(&self) -> &str {
        self.city_id.as_ref()
    }

    pub fn get_district_id(&self) -> &str {
        self.district_id.as_ref()
    }

    pub fn get_street(&self) -> &str {
        self.street.as_ref()
    }

    pub fn get_zip_code(&self) -> &str {
        self.zip_code.as_ref()
    }
}
