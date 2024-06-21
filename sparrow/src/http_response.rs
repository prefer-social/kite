use anyhow::Result;
use spin_sdk::http::{IntoResponse, Response};

pub struct HttpResponse;

impl HttpResponse {
    pub async fn unauthorized() -> Result<Response> {
        let m = r#"{"message": "Unauthorized (401)"}"#;
        Ok(Response::builder()
            .status(401)
            .header("Content-Type", "Application/json")
            .body(m)
            .build())
    }

    pub async fn not_found() -> anyhow::Result<Response> {
        let m = r#"{"message": "Not Found (404)"}"#;
        Ok(Response::builder()
            .status(404)
            .header("Content-Type", "Application/json")
            .body(m)
            .build())
    }

    pub async fn not_implemented() -> Result<Response> {
        let m = r#"{"message": "Not Implemented Yet (401)"}"#;
        Ok(Response::builder()
            .status(401)
            .header("Content-Type", "Application/json")
            .body(m)
            .build())
    }

    pub async fn invalid_request() -> Result<Response> {
        let m = r#"{"message": "Invalid Request (400)"}"#;
        Ok(Response::builder()
            .status(400)
            .header("Content-Type", "Application/json")
            .body(m)
            .build())
    }

    pub async fn method_not_allowed() -> Result<Response> {
        let m = r#"{"message": "Method Not Allowed (405)"}"#;
        Ok(Response::builder()
            .status(405)
            .header("Content-Type", "Application/json")
            .body(m)
            .build())
    }
}
