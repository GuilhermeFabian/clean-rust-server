use super::entities::{ PokemonName, PokemonNumber, PokemonTypes };

struct Request {
    number: u16,
    name: String,
    types: Vec<String>,
}

enum Response {
    Ok(u16),
    BadRequest,
}

fn execute(request: Request) -> Response {
    match (
        PokemonNumber::try_from(request.number),
        PokemonName::try_from(request.name),
        PokemonTypes::try_from(request.types),
    ) {
        (Ok(number), Ok(_), Ok(_)) => Response::Ok(u16::from(number)),
        _ => Response::BadRequest,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_the_pokemon_number_otherwise() {
        let number = 25;

        let req = Request {
            number,
            name: String::from("Pikachu"),
            types: vec![String::from("Electric")],
        };

        let response = execute(req);

        match response {
            Response::Ok(response_number) => assert_eq!(response_number, number),
            _ => unreachable!(),
        };
    }

    #[test]
    fn it_should_return_a_bad_request_error_when_request_is_invalid() {
        let request = Request {
            number: 25,
            name: String::from(""),
            types: vec![String::from("Electric")],
        };

        let response = execute(request);

        match response {
            Response::BadRequest => {}
            _ => unreachable!(),
        }
    }
}
