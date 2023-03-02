use std::sync::Arc;

use crate::repositories::pokemon::{Repository, FetchManyError};

pub struct Request {
  pub name: String,
}

pub struct Response {
  pub number: u16,
  pub name: String,
  pub types: Vec<String>,
}

pub enum Error {
  Unknown,
}

pub fn execute(repo: Arc<dyn Repository>, req: Request) -> Result<Vec<Response>, Error> {
  match repo.fetch_by_name(req.name) {
    Ok(pokemons) => Ok(pokemons
      .into_iter()
      .map(|p| Response {
        number: u16::from(p.number),
        name: String::from(p.name),
        types: Vec::<String>::from(p.types),
      })
      .collect::<Vec<Response>>()),
    Err(FetchManyError::Unknown) => Err(Error::Unknown),
  }
}

/*#[cfg(test)]
mod tests {
  use super::*;
  use crate::{domain::entities::{PokemonName, PokemonNumber, PokemonTypes},
    repositories::pokemon::InMemoryRepository};

  #[test]
  fn it_should_return_an_unknown_error_when_an_unexpected_error_happens() {
    let repo = Arc::new(InMemoryRepository::new().with_error());
    let req = 

    let res = execute(repo, );

    match res {
      Err(Error::Unknown) => {}
      _ => unreachable!(),
    }
  }

  #[test]
  fn it_should_return_all_the_pokemons_ordered_by_increasing_number_otherwise() {
    let repo = Arc::new(InMemoryRepository::new());
    repo.insert_one(
      PokemonNumber::pikachu(),
      PokemonName::pikachu(),
      PokemonTypes::pikachu(),
    ).ok();

    repo.insert_one(
      PokemonNumber::charmander(),
      PokemonName::charmander(),
      PokemonTypes::charmander(),
    ).ok();

    let res = execute(repo);

    match res {
      Ok(res) => {
        assert_eq!(res[0].number, u16::from(PokemonNumber::charmander()));
        assert_eq!(res[0].name, String::from(PokemonName::charmander()));
        assert_eq!(res[0].types, Vec::<String>::from(PokemonTypes::charmander()));
        assert_eq!(res[1].number, u16::from(PokemonNumber::pikachu()));
        assert_eq!(res[1].name, String::from(PokemonName::pikachu()));
        assert_eq!(res[1].types, Vec::<String>::from(PokemonTypes::pikachu()));
      }
      _ => unreachable!(),
    }
  }
}*/