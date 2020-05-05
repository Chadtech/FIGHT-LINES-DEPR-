use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use fightlines_moves::moves::Move;
use std::collections::HashMap;
use std::io;

//
struct Model {
    games: HashMap<String, Game>,
}

struct Game {
    name: String,
    players: Vec<Player>,
    round: i64,
}
struct Player {
    name: String,
    /// None means the player has not submitted moves for this turn
    /// Some(...) means they have, including the possibility that
    /// they submitted an empty list of moves.
    moves: Option<Vec<Move>>,
}

// Responder Objects
// GET /
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

// GET /again
async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello World Again")
}

// GET /games/count, we also pass in the state
async fn game_count(model: web::Data<Model>) -> impl Responder {
    HttpResponse::Ok().body(model.games.len().to_string())
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .data(Model {
                games: HashMap::new(),
            })
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
            .route("/games/count", web::get().to(game_count))
    })
    .bind("127.0.0.1:2943")?
    .run()
    .await
}
