use std::sync::Arc;

use serde::{Serialize, Deserialize};

use crate::{domain::fetch_one_by_number_pokemon, repositories::pokemon::Repository};

use crate::api::Status;

#[derive(Deserialize)]
struct Request {
  number: u16,
}

#[derive(Serialize)]
struct Response {
  number: u16,
  name: String,
  types: Vec<String>,
}

pub fn serve_req(repo: Arc<dyn Repository>, req: &rouille::Request) -> rouille::Response {
  let req = match rouille::input::json_input::<Request>(req) {
    Ok(req) => fetch_one_by_number_pokemon::Request {
      number: req.number,
    },
    _ => return rouille::Response::from(Status::BadRequest),
  };

  match fetch_one_by_number_pokemon::execute(repo, req) {
    Ok(fetch_one_by_number_pokemon::Response {
      number,
      name,
      types,
    }) => rouille::Response::json(&Response {
      number,
      name,
      types,
    }),
    Err(fetch_one_by_number_pokemon::Error::BadRequest) => rouille::Response::from(Status::BadRequest),
    Err(fetch_one_by_number_pokemon::Error::NotFound) => rouille::Response::from(Status::NotFound),
    Err(fetch_one_by_number_pokemon::Error::Unknown) => rouille::Response::from(Status::InternalServerError),
  }
}

pub fn serve(repo: Arc<dyn Repository>, number: u16) -> rouille::Response {
  let req = fetch_one_by_number_pokemon::Request { number };

  match fetch_one_by_number_pokemon::execute(repo, req) {
    Ok(fetch_one_by_number_pokemon::Response {
      number,
      name,
      types,
    }) => rouille::Response::json(&Response {
      number,
      name,
      types,
    }),
    Err(fetch_one_by_number_pokemon::Error::BadRequest) => rouille::Response::from(Status::BadRequest),
    Err(fetch_one_by_number_pokemon::Error::NotFound) => rouille::Response::from(Status::NotFound),
    Err(fetch_one_by_number_pokemon::Error::Unknown) => rouille::Response::from(Status::InternalServerError),
  }
}