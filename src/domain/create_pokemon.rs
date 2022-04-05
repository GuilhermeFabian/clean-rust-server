use super::entities::{ PokemonName, PokemonNumber, PokemonTypes };
use crate::repositories::pokemon::{ InMemoryRepository, Repository, Insert };

struct Request {
    number: u16,
    name: String,
    types: Vec<String>,
}

enum Response {
    Ok(u16),
    BadRequest,
    Conflict,
    Error,
}

fn execute(repository: &mut dyn Repository, request: Request) -> Response {
    match (
        PokemonNumber::try_from(request.number),
        PokemonName::try_from(request.name),
        PokemonTypes::try_from(request.types),
    ) {
        (Ok(number), Ok(name), Ok(types)) => match repository.insert(number, name, types) {
            Insert::Ok(number) => Response::Ok(u16::from(number)),
            Insert::Conflict => Response::Conflict,
            Insert::Error => Response::Error,
        },
        _ => Response::BadRequest,
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn it_should_return_the_pokemon_number_otherwise() {
        let number = 25;
        let mut repository = InMemoryRepository::new();

        let req = Request {
            number,
            name: String::from("Pikachu"),
            types: vec![String::from("Electric")],
        };

        let response = execute(&mut repository, req);

        match response {
            Response::Ok(response_number) => assert_eq!(response_number, number),
            _ => unreachable!(),
        };
    }

    #[test]
    fn it_should_return_a_bad_request_error_when_request_is_invalid() {
        let mut repository = InMemoryRepository::new();

        let request = Request {
            number: 25,
            name: String::from(""),
            types: vec![String::from("Electric")],
        };

        let response = execute(&mut repository, request);

        match response {
            Response::BadRequest => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_a_conflict_error_when_pokemon_number_already_exists() {
        let number = PokemonNumber::try_from(25).unwrap();
        let name = PokemonName::try_from(String::from("Pikachu")).unwrap();
        let types = PokemonTypes::try_from(vec![String::from("Electric")]).unwrap();
        let mut repository = InMemoryRepository::new();

        repository.insert(number, name, types);

        let request = Request {
            number: 25,
            name: String::from("Charmander"),
            types: vec![String::from("Fire")],
        };

        let response = execute(&mut repository, request);

        match response {
            Response::Conflict => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn it_should_return_an_error_when_an_unexpected_error_happens() {
        let mut repository = InMemoryRepository::new().with_error();
        let number = 25;
        let request = Request {
            number,
            name: String::from("Pikachu"),
            types: vec![String::from("Electric")],
        };

        let response = execute(&mut repository, request);

        match response {
            Response::Error => {}
            _ => unreachable!(),
        }
    }
}
