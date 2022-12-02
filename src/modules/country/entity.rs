#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCountryPayload {
    name: String,
    code: String,
}

impl CreateCountryPayload {
    pub fn get_name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn get_code(&self) -> &str {
        self.code.as_ref()
    }
}
