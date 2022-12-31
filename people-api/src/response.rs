use anyhow::Result;
use spin_sdk::http::Response;

/// A generic error handler
fn quick_response(status: http::StatusCode) -> Result<Response> {
    Ok(http::Response::builder().status(status).body(None)?)
}

pub(crate) fn method_not_allowed() -> Result<Response> {
    quick_response(http::StatusCode::METHOD_NOT_ALLOWED)
}

/// Returns a 404 not found response
pub(crate) fn not_found() -> Result<Response> {
    quick_response(http::StatusCode::NOT_FOUND)
}

pub(crate) fn bad_request() -> Result<Response> {
    quick_response(http::StatusCode::BAD_REQUEST)
}

/// Returns a successful (200) response
pub(crate) fn ok(payload: String) -> Result<Response> {
    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(Some(payload.into()))?)
}

/// Returns a unsuccessful (500) response
pub(crate) fn internal_server_error() -> Result<Response> {
    quick_response(http::StatusCode::INTERNAL_SERVER_ERROR)
}
