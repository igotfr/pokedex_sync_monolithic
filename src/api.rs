mod create_one_pokemon;
mod fetch_all_pokemons;
mod fetch_by_name_pokemons;
mod fetch_one_by_number_pokemon;
mod delete_one_by_number_pokemon;
mod health;

use std::sync::Arc;

use crate::repositories::pokemon::Repository;

pub fn serve(url: &str, repo: Arc<dyn Repository>) {
  rouille::start_server(url, move |req| {
    router!(req,
      (GET) (/health) => {
        health::serve()
      },
      (GET) (/) => {
        fetch_all_pokemons::serve(repo.clone())
      },
      (GET) (/{name: String}) => {
        fetch_by_name_pokemons::serve(repo.clone(), name)
      },
      (GET) (/{number: u16}) => {
        fetch_one_by_number_pokemon::serve(repo.clone(), number)
      },
      (GET) (/) => {
        fetch_one_by_number_pokemon::serve_req(repo.clone(), req)
      },
      (POST) (/) => {
        create_one_pokemon::serve(repo.clone(), req)
      },
      (DELETE) (/{number: u16}) => {
        delete_one_by_number_pokemon::serve(repo.clone(), number)
      },
      (DELETE) (/) => {
        delete_one_by_number_pokemon::serve_req(repo.clone(), req)
      },
      _ => {
        rouille::Response::from(Status::NotFound)
      }
    )
  });
}

enum Status {
  Ok,
  BadRequest,
  NotFound,
  Conflict,
  InternalServerError,
}

impl From<Status> for rouille::Response {
  fn from(status: Status) -> Self {
    let status_code = match status {
      Status::Ok => 200,
      Status::BadRequest => 400,
      Status::NotFound => 404,
      Status::Conflict => 409,
      Status::InternalServerError => 500,
    };

    Self {
      status_code,
      headers: vec![],
      data: rouille::ResponseBody::empty(),
      upgrade: None,
    }
  }
}