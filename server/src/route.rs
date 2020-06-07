use actix_web::{web, HttpResponse, Responder};
use code_gen::protos::game_request::GameRequest;
use std::sync::Mutex;

use crate::domain::game;
use crate::domain::model::FormData;
use crate::domain::model::Model;
use protobuf::{parse_from_bytes, ProtobufResult};
use std::fs;

/// Responder Objects
/// GET /
pub async fn index() -> impl Responder {
    match fs::read_to_string("../ui/index.html") {
        Ok(index_file) => HttpResponse::Ok().body(index_file),
        Err(_error) => HttpResponse::NotFound().body("File not found"),
    }
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
pub async fn post_game(mut body: String, model: web::Data<Mutex<Model>>) -> impl Responder {
    // let mut data = model.lock().unwrap();
    // data.add_game(game::init(&form.game_name));
    // let mut gameX = GameRequest::new();
    match hex::decode(body) {
        Ok(payload) => {
            // let gamex: ProtobufResult<GameRequest> = parse_from_bytes(payload);
            // println!("Body {:?}!", payload)
        }
        Err(_) => {}
    }
    // let game_r = gameX::merge_from_bytes(payload);
    "Hello from POST Request"
}

pub async fn proto_test() -> impl Responder {
    let mut out_response = GameRequest::new();
    out_response.set_gameName("Response Game".to_string());
    out_response.set_numPlayers(55);
    HttpResponse::Ok().body(out_response.take_gameName())
}
