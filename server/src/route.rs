use actix_web::{web, HttpResponse, Responder};
use shared::game_request::GameRequest;
use std::sync::Mutex;

use crate::domain::game;
use crate::domain::model::Model;
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
pub async fn post_game(body: String, model: web::Data<Mutex<Model>>) -> impl Responder {
    // Decode the hex message from the server
    // hex decode returns a Result type, needs to match
    match hex::decode(body) {
        Ok(payload) => {
            // Create protobuf Result type from parse_from_bytes
            let result: GameRequest = GameRequest::from_bytes(payload);
            let mut data = model.lock().unwrap();
            let game_id: u64 = data.add_game(game::init(result.game_name()));
            format!("Hello from POST {:?}", game_id)
        }
        Err(error) => error.to_string(),
    }
}
