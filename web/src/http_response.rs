//! Http resonse tyeps for Mastodon service (TODO: Moce to kite/)
//!
//! TODO: This should be moved to under Kite.
//!

use anyhow::Result;
use spin_sdk::http::Response;

/// HttpResponses for Sparrow
pub struct HttpResponse;

impl HttpResponse {
    /// 201 Created    
    pub fn created() -> Result<Response> {
        let m = r#"{"message": "201 Created"}"#;
        Ok(Response::builder()
            .status(201)
            .header("Content-Type", "Application/json")
            .body(m)
            .build())
    }

    /// 202 Accepted  
    pub fn accepted() -> Result<Response> {
        let m = r#"{"message": "202 Accepted"}"#;
        Ok(Response::builder()
            .status(202)
            .header("Content-Type", "Application/json")
            .body(m)
            .build())
    }

    /// 400 Bad Request
    pub fn invalid_request() -> Result<Response> {
        let m = r#"{"message": "400 Bad Request"}"#;
        Ok(Response::builder()
            .status(400)
            .header("Content-Type", "Application/json")
            .body(m)
            .build())
    }

    /// 401 Unauthorized    
    /// Invalid or missing Authorization header  
    pub fn unauthorized() -> Result<Response> {
        let m = r#"{ "error": "The access token is invalid" }"#;
        Ok(Response::builder()
            .status(401)
            .header("Content-Type", "Application/json")
            .body(m)
            .build())
    }

    /// 403 Forbidden    
    pub fn forbidden() -> anyhow::Result<Response> {
        let m = r#"{ "error": "This action is not allowed" }"#;
        Ok(Response::builder()
            .status(403)
            .header("Content-Type", "Application/json")
            .body(m)
            .build())
    }

    /// 404 Not Found  
    pub fn not_found() -> anyhow::Result<Response> {
        let m = r#"{"message": "404 Not Found"}"#;
        Ok(Response::builder()
            .status(404)
            .header("Content-Type", "Application/json")
            .body(m)
            .build())
    }

    /// 405 Method Not Allowed  
    pub fn method_not_allowed() -> Result<Response> {
        let m = r#"{"message": "405 Method Not Allowed"}"#;
        Ok(Response::builder()
            .status(405)
            .header("Content-Type", "Application/json")
            .body(m)
            .build())
    }

    /// 406 Not Acceptable  
    /// The HTTP 406 Not Acceptable client error response status code indicates that the server could not produce a response matching the list of acceptable values defined in the request's proactive content negotiation headers and that the server was unwilling to supply a default representation.
    pub fn not_acceptable() -> Result<Response> {
        let m = r#"{"message": "406 Not Acceptable"}"#;
        Ok(Response::builder()
            .status(406)
            .header("Content-Type", "Application/json")
            .body(m)
            .build())
    }

    /// 410 Gone  
    pub fn gone() -> Result<Response> {
        let m = r#"{"message": "410 Gone"}"#;
        Ok(Response::builder()
            .status(410)
            .header("Content-Type", "Application/json")
            .body(m)
            .build())
    }

    /// 418 I'm a teapot
    pub fn i_am_a_teapot() -> Result<Response> {
        let m = r#"{"message": "418 I'm a teapot"}"#;
        Ok(Response::builder()
            .status(418)
            .header("Content-Type", "Application/json")
            .body(m)
            .build())
    }

    /// 422 Unprocessable Content
    /// Token does not have an authorized user
    pub fn unprocessable_content() -> Result<Response> {
        let m = r#"{ "error": "This method requires an authenticated user" }"#;
        Ok(Response::builder()
            .status(422)
            .header("Content-Type", "Application/json")
            .body(m)
            .build())
    }

    /// 429 Too Many Requests  
    pub fn too_many_requests() -> Result<Response> {
        let m = r#"{"message": "429 Too Many Requests"}"#;
        Ok(Response::builder()
            .status(429)
            .header("Content-Type", "Application/json")
            .body(m)
            .build())
    }

    /// 500 Internal Server Error    
    pub fn internal_server_error() -> Result<Response> {
        let m = r#"{"message": "500 Internal Server Error"}"#;
        Ok(Response::builder()
            .status(500)
            .header("Content-Type", "Application/json")
            .body(m)
            .build())
    }

    /// 501 Not Implemented  
    pub fn not_implemented() -> Result<Response> {
        let m = r#"{"message": "501 Not Implemented"}"#;
        Ok(Response::builder()
            .status(501)
            .header("Content-Type", "Application/json")
            .body(m)
            .build())
    }
}
