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

/// Returns a 400 bad request response
pub fn bad_request() -> Result<Response> {
    quick_response(http::StatusCode::BAD_REQUEST)
}

/// Returns a 404 not found response
pub fn not_found() -> Result<Response> {
    Ok(http::Response::builder()
        .status(http::StatusCode::NOT_FOUND)
        .header(http::header::CONTENT_TYPE, "text/plain")
        .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .body(Some("Not Found".into()))?)
}

/// Returns a 200 response
pub fn ok(payload: String) -> Result<Response> {
    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "application/json")
        .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .body(Some(payload.into()))?)
}

/// Returns a 500 response
pub fn internal_server_error() -> Result<Response> {
    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "text/plain")
        .body(Some("Something went wrong".into()))?)
}
