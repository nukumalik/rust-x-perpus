#![allow(dead_code)]
use std::fmt::Display;

pub struct PaginationSort {
    limit: Option<u32>,
    offset: Option<u32>,
    order_by: Option<String>,
    order_dir: Option<String>,
}

impl PaginationSort {
    pub fn get_limit(&self) -> Option<u32> {
        self.limit
    }

    pub fn set_limit(&mut self, limit: Option<u32>) {
        self.limit = limit;
    }

    pub fn get_offset(&self) -> Option<u32> {
        self.offset
    }

    pub fn set_offset(&mut self, offset: Option<u32>) {
        self.offset = offset;
    }

    pub fn get_order_by(&self) -> Option<&String> {
        self.order_by.as_ref()
    }

    pub fn set_order_by(&mut self, order_by: Option<String>) {
        self.order_by = order_by;
    }

    pub fn get_order_dir(&self) -> Option<&String> {
        self.order_dir.as_ref()
    }

    pub fn set_order_dir(&mut self, order_dir: Option<String>) {
        self.order_dir = order_dir;
    }
}

impl Default for PaginationSort {
    fn default() -> Self {
        Self {
            limit: Some(25),
            offset: Some(0),
            order_by: Some("created_at".to_string()),
            order_dir: Some("desc".to_string()),
        }
    }
}

impl Display for PaginationSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ORDER BY {} {} LIMIT {} OFFSET {}",
            if self.order_by.is_none() {
                "created_at"
            } else {
                self.order_by.as_ref().unwrap()
            },
            if self.order_dir.is_none() {
                "asc"
            } else {
                self.order_dir.as_ref().unwrap()
            },
            if self.limit.is_none() {
                25
            } else {
                self.limit.unwrap()
            },
            if self.offset.is_none() {
                0
            } else {
                self.offset.unwrap()
            }
        )
    }
}
