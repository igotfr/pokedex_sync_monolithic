use serde::Serialize;

use std::sync::Arc;

use crate::{api::Status, domain::fetch_by_name_pokemons, repositories::pokemon::Repository};

#[derive(Serialize)]
struct Response {
  number: u16,
  name: String,
  types: Vec<String>,
}

pub fn serve(repo: Arc<dyn Repository>, name: String) -> rouille::Response {
  let req = fetch_by_name_pokemons::Request { name };
  match fetch_by_name_pokemons::execute(repo, req) {
    Ok(res) => rouille::Response::json(
      &res.into_iter()
        .map(|p| Response {
          number: p.number,
          name: p.name,
          types: p.types,
        })
        .collect::<Vec<Response>>(),
    ),
    Err(fetch_by_name_pokemons::Error::Unknown) => {
      rouille::Response::from(Status::InternalServerError)
    }
  }
}