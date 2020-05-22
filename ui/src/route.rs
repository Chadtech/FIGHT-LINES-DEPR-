use seed::Url;

////////////////////////////////////////////////////////////////
// TYPES //
////////////////////////////////////////////////////////////////

#[derive(Copy, Clone)]
pub enum Route {
    Title,
    StartGame,
}

////////////////////////////////////////////////////////////////
// API //
////////////////////////////////////////////////////////////////

impl Route {
    pub fn path(&self) -> Vec<&str> {
        match self {
            Route::Title => vec![],
            Route::StartGame => vec!["start-game"],
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
            _ => None,
        },
    }
}
