#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProvincePayload {
    name: String,
    country_id: String,
}

impl CreateProvincePayload {
    pub fn get_name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn get_country_id(&self) -> &str {
        self.country_id.as_ref()
    }
}
