use seed::Url;
use std::fmt;
use std::slice::Iter;

////////////////////////////////////////////////////////////////
// TYPES //
////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub enum Route {
    Title,
    StartGame,
    Demo,
    Lobby(String),
    Game(String),
}

////////////////////////////////////////////////////////////////
// API //
////////////////////////////////////////////////////////////////

impl Route {
    pub fn path(&self) -> Vec<&str> {
        match self {
            Route::Title => vec![],
            Route::StartGame => vec!["start-game"],
            Route::Demo => vec!["demo"],
            Route::Lobby(game_id) => vec!["game", game_id.as_str(), "lobby"],
            Route::Game(game_id) => vec!["game", game_id.as_str()],
        }
    }
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();

        buf.push('/');

        buf.push_str(&self.path().join("/"));

        write!(f, "{}", buf)
    }
}

pub fn parse(url: Url) -> Option<Route> {
    let mut path = url.path().iter();

    fn parse_game_route(mut remaining_path: Iter<String>) -> Option<Route> {
        match remaining_path.next() {
            None => None,
            Some(game_id_ref) => {
                let game_id = game_id_ref.clone();
                match remaining_path.next() {
                    None => Some(Route::Game(game_id)),
                    Some(first) => match first.as_str() {
                        "lobby" => Some(Route::Lobby(game_id)),
                        _ => None,
                    },
                }
            }
        }
    }

    match path.next() {
        None => Some(Route::Title),
        Some(first) => match first.as_str() {
            "start-game" => Some(Route::StartGame),
            "demo" => Some(Route::Demo),
            "game" => parse_game_route(path),

            _ => None,
        },
    }
}
