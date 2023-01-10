use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

use rest_api::handlers::not_found;

/// A generic 404 handler for routes that are not implemented
#[http_component]
fn not_found_404(_req: Request) -> Result<Response> {
    not_found()
}
