use anyhow::Result;
use serde::de::DeserializeOwned;
use spin_sdk::http::Request;

use crate::util::{from_bytes, get_id_from_path};

pub enum Api<Model> {
    Create(Model),
    FindAll,
    FindById(i32),
    Update(i32, Model),
    Delete(i32),
    BadRequest,
    NotFound,
    MethodNotAllowed,
    InternalServerError,
}

/// Gets the correct API response based on the Request object
pub fn get_api_from_request<Model: DeserializeOwned + 'static>(req: Request) -> Result<Api<Model>> {
    let path_info = req.headers().get("spin-path-info");

    Ok(match path_info {
        Some(path) => match *req.method() {
            http::Method::POST => {
                let json: Result<Model> = from_bytes(&req);
                match json {
                    Ok(model) => Api::Create(model),
                    Err(err) => {
                        println!("{:#?}", err);
                        Api::BadRequest
                    }
                }
            }
            http::Method::GET => match get_id_from_path(path) {
                Ok(Some(id)) => Api::FindById(id),
                Ok(None) => Api::FindAll,
                Err(_) => Api::NotFound,
            },
            http::Method::PUT => match get_id_from_path(path) {
                Ok(Some(id)) => {
                    let json: Result<Model> = from_bytes(&req);
                    match json {
                        Ok(model) => Api::Update(id, model),
                        Err(_) => Api::BadRequest,
                    }
                }
                Ok(None) => Api::NotFound,
                Err(_) => Api::NotFound,
            },
            http::Method::DELETE => {
                let id = get_id_from_path(path);
    

                match id {
                    Ok(Some(id)) => Api::Delete(id),
                    Ok(None) => Api::NotFound,
                    Err(_) => Api::NotFound,
                }
            }
            _ => Api::MethodNotAllowed,
        },
        None => Api::InternalServerError,
    })
}
