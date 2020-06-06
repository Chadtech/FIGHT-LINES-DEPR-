use seed::Url;

////////////////////////////////////////////////////////////////
// TYPES //
////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub enum Route {
    Title,
    StartGame,
    Demo,
    Lobby(String),
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
        }
    }
    pub fn to_string(self) -> String {
        let mut buf = String::new();

        buf.push('/');

        buf.push_str(&self.path().join("/"));

        buf
    }
}

pub fn parse(url: Url) -> Option<Route> {
    let mut path = url.path().iter();

    match path.next() {
        None => Some(Route::Title),
        Some(first) => match first.as_str() {
            "start-game" => Some(Route::StartGame),
            "demo" => Some(Route::Demo),
            _ => None,
        },
    }
}
