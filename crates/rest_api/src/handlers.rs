use anyhow::Result;
use spin_sdk::http::Response;

/// A generic error handler
fn quick_response(status: http::StatusCode) -> Result<Response> {
    Ok(http::Response::builder().status(status).body(None)?)
}

/// Returns a 405 method not found response
pub fn method_not_allowed() -> Result<Response> {
    quick_response(http::StatusCode::METHOD_NOT_ALLOWED)
}

/// Returns a 404 not found response
pub fn not_found() -> Result<Response> {
    quick_response(http::StatusCode::NOT_FOUND)
}

/// Returns a 400 not found response
pub fn bad_request() -> Result<Response> {
    quick_response(http::StatusCode::BAD_REQUEST)
}

/// Returns a 200 response
pub fn ok(payload: String) -> Result<Response> {
    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(Some(payload.into()))?)
}

/// Returns a 500 response
pub fn internal_server_error() -> Result<Response> {
    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "text/plain")
        .body(Some("Something went wrong".into()))?)
}
