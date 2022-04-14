use std::sync::Arc;

use rouille;
use serde::{Serialize, Deserialize};

use crate::{api::Status, repositories::pokemon::Repository, domain::create_pokemon};

#[derive(Serialize)]
struct Response {
    number: u16,
}

#[derive(Deserialize)]
struct Request {
    number: u16,
    name: String,
    types: Vec<String>,
}

pub fn serve(repo: Arc<dyn Repository>, req: &rouille::Request) -> rouille::Response {
    let req = match rouille::input::json_input::<Request>(req) {
        Ok(req) => create_pokemon::Request {
            number: req.number,
            name: req.name,
            types: req.types,
        },
        _ => return rouille::Response::from(Status::BadRequest),
    };

    match create_pokemon::execute(repo, req) {
        create_pokemon::Response::Ok(number) => rouille::Response::json(&Response { number }),
        create_pokemon::Response::BadRequest => rouille::Response::from(Status::BadRequest),
        create_pokemon::Response::Conflict => rouille::Response::from(Status::Conflict),
        create_pokemon::Response::Error => rouille::Response::from(Status::InternalServerError),
    }
}
