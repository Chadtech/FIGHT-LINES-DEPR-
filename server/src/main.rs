
#![feature(proc_macro_hygiene, decl_macro)]
use fightlines_moves::moves::{Move};
use std::collections::HashMap;
use rocket::State;

#[macro_use] extern crate rocket;

struct Model {
    games: HashMap<String, Game>
}

struct Game {
    name: String,
    players: Vec<Player>,
    round: i64
}

struct Player {
    name: String,
    /// None means the player has not submitted moves for this turn
    /// Some(...) means they have, including the possibility that
    /// they submitted an empty list of moves.
    moves: Option<Vec<Move>>
}


#[get("/games/count")]
fn game_count(model: State<Model>) -> String {
    return model.games.len().to_string();
}

fn start(model: State<Model>) -> String {


}

fn main() {
    rocket::ignite()
        .manage(Model { games: HashMap::new() })
        .mount(
            "/",
            routes![
                game_count
            ]
        ).launch();
}