#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDistrictPayload {
    name: String,
    city_id: String,
}

impl CreateDistrictPayload {
    pub fn get_name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn get_city_id(&self) -> &str {
        self.city_id.as_ref()
    }
}
