use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

/// A simple Spin HTTP component.
#[http_component]
fn not_found(_req: Request) -> Result<Response> {
    Ok(http::Response::builder()
        .status(404)
        .header("Content-Type", "text/plain")
        .body(Some("Not Found".into()))?)
}
