use std::sync::Arc;

use serde::Deserialize;

use crate::{domain::delete_one_by_number_pokemon, repositories::pokemon::Repository};

use super::Status;

#[derive(Deserialize)]
struct Request {
  number: u16,
}

pub fn serve_req(repo: Arc<dyn Repository>, req: &rouille::Request) -> rouille::Response {
  let req = match rouille::input::json_input::<Request>(req) {
    Ok(req) => delete_one_by_number_pokemon::Request {
      number: req.number,
    },
    _ => return rouille::Response::from(Status::BadRequest),
  };

  match delete_one_by_number_pokemon::execute(repo, req) {
    Ok(()) => rouille::Response::from(Status::Ok),
    Err(delete_one_by_number_pokemon::Error::BadRequest) => rouille::Response::from(Status::BadRequest),
    Err(delete_one_by_number_pokemon::Error::NotFound) => rouille::Response::from(Status::NotFound),
    Err(delete_one_by_number_pokemon::Error::Unknown) => rouille::Response::from(Status::InternalServerError),
  }
}

pub fn serve(repo: Arc<dyn Repository>, number: u16) -> rouille::Response {
  let req = delete_one_by_number_pokemon::Request { number };
  match delete_one_by_number_pokemon::execute(repo, req) {
    Ok(()) => rouille::Response::from(Status::Ok),
    Err(delete_one_by_number_pokemon::Error::BadRequest) => rouille::Response::from(Status::BadRequest),
    Err(delete_one_by_number_pokemon::Error::NotFound) => rouille::Response::from(Status::NotFound),
    Err(delete_one_by_number_pokemon::Error::Unknown) => rouille::Response::from(Status::InternalServerError),
  }
}