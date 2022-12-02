#![allow(dead_code)]
use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonResponse<T, E> {
    code: &'static str,
    status: u16,
    message: &'static str,
    data: Option<T>,
    error: Option<E>,
}

impl<T, E> JsonResponse<T, E> {
    pub fn ok(message: &'static str, data: Option<T>) -> Self {
        Self {
            code: "OK",
            status: StatusCode::OK.as_u16(),
            message,
            data,
            error: None,
        }
    }

    pub fn created(message: &'static str, data: Option<T>) -> Self {
        Self {
            code: "CREATED",
            status: StatusCode::CREATED.as_u16(),
            message,
            data,
            error: None,
        }
    }

    pub fn bad_request(message: &'static str, error: Option<E>) -> Self {
        Self {
            code: "BAD_REQUEST",
            status: StatusCode::BAD_REQUEST.as_u16(),
            message,
            data: None,
            error,
        }
    }

    pub fn not_found(message: &'static str, error: Option<E>) -> Self {
        Self {
            code: "NOT_FOUND",
            status: StatusCode::NOT_FOUND.as_u16(),
            message,
            data: None,
            error,
        }
    }

    pub fn bad_gateway(message: &'static str, error: Option<E>) -> Self {
        Self {
            code: "BAD_GATEWAY",
            status: StatusCode::BAD_GATEWAY.as_u16(),
            message,
            data: None,
            error,
        }
    }
}
