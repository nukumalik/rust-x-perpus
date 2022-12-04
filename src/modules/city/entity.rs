#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCityPayload {
    name: String,
    province_id: String,
}

impl CreateCityPayload {
    pub fn get_name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn get_province_id(&self) -> &str {
        self.province_id.as_ref()
    }
}
