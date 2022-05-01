use rocket::{http::Status, Request};
use serde_derive::Serialize;

use crate::util::get_string;

#[derive(Debug, Serialize)]
pub struct Response {
    pub code: u16,
    pub description: Option<String>,
}

impl Response {
    pub fn ok(response: &'static Option<&'static str>) -> Self {
        Self {
            code: Status::Ok.code,
            description: get_string(response),
        }
    }

    pub fn teapot(response: &'static Option<&'static str>) -> Self {
        Self {
            code: Status::ImATeapot.code,
            description: get_string(response),
        }
    }

    pub fn not_found(req: &Request) -> Self {
        Self {
            code: Status::NotFound.code,
            description: Some(format!("The requested page is invalid: {}", req.uri())),
        }
    }
}
