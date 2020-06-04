use actix_web::{web, HttpResponse, Responder};
use code_gen::protos::game_request::GameRequest;
use std::sync::Mutex;

use crate::domain::game;
use crate::domain::model::Model;
use protobuf::{parse_from_bytes, ProtobufResult};

/// Responder Objects
/// GET /
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

/// GET /again
pub async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello World Again")
}

/// GET /games/count, we also pass in the state
pub async fn game_count(model: web::Data<Model>) -> impl Responder {
    HttpResponse::Ok().body(model.games_count().to_string())
}

/// POST /game/create This
/// function will be called from a post request
pub async fn post_game(body: String, model: web::Data<Mutex<Model>>) -> impl Responder {
    // Decode the hex message from the server
    // hex decode returns a Result type, needs to match
    match hex::decode(body) {
        Ok(payload) => {
            // Create protobuf Result type from parse_from_bytes
            let result: ProtobufResult<GameRequest> = parse_from_bytes(&payload);

            match result {
                Ok(output) => {
                    // unlock mutex to add new Game Object
                    let mut data = model.lock().unwrap();
                    data.add_game(game::init(&output.gameName.to_string()));
                }
                Err(_) => {}
            }
        }
        Err(_) => {}
    }
    "Hello from POST"
}

pub async fn proto_test() -> impl Responder {
    let mut out_response = GameRequest::new();
    out_response.set_gameName("Response Game".to_string());
    out_response.set_numPlayers(55);
    HttpResponse::Ok().body(out_response.take_gameName())
}
