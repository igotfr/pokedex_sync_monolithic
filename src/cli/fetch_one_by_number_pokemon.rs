use std::sync::Arc;

use crate::{repositories::pokemon::Repository, domain::fetch_one_by_number_pokemon};

use crate::cli::prompt_number;

#[derive(Debug)]
struct Response {
    number: u16,
    name: String,
    types: Vec<String>,
}

pub fn run(repo: Arc<dyn Repository>) {
    let number = prompt_number();
    
    let req = match number {
        Ok(number) => fetch_one_by_number_pokemon::Request { number },
        _ => {
            println!("An error occurred during the prompt");
            return
        }
    };
    match fetch_one_by_number_pokemon::execute(repo, req) {
        Ok(res) => println!("{:?}", Response {
            number: res.number,
            name: res.name,
            types: res.types,
        }),
        Err(fetch_one_by_number_pokemon::Error::BadRequest) => println!("The request is invalid"),
        Err(fetch_one_by_number_pokemon::Error::NotFound) => println!("The pokemon does not exist"),
        Err(fetch_one_by_number_pokemon::Error::Unknown) => println!("An unknown error occurred"),
    }
}