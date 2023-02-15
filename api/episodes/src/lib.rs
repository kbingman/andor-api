use anyhow::Result;
use rest_api::api::{get_api_from_request, Api};
use rest_api::handlers::{bad_request, internal_server_error, method_not_allowed, not_found};
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

use crate::db::EpisodeDb;
use crate::models::Episode;

mod db;
mod handlers;
mod models;

#[http_component]
fn episode_api(req: Request) -> Result<Response> {
    let uri = spin_sdk::config::get("postgres_uri")?;
    let api: Api<Episode> = get_api_from_request(&req)?;
    let db = EpisodeDb::new(uri);

    match api {
        Api::Create(model) => handlers::create(db, model),
        Api::FindAll => handlers::find_all(db),
        Api::FindById(id) => handlers::find_by_id(db, id),
        Api::Update(id, model) => handlers::update(db, id, model),
        Api::Delete(id) => handlers::delete(db, id),
        Api::MethodNotAllowed => method_not_allowed(),
        Api::NotFound => not_found(),
        Api::BadRequest => bad_request(),
        Api::InternalServerError => internal_server_error(),
    }
}
