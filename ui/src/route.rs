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
    pub fn as_str(self) -> &'static str {
        match self {
            Route::Title => "",
            Route::StartGame => "start-game",
        }
    }
}

pub fn parse(url: Url) -> Option<Route> {
    if url.path().is_empty() {
        return Some(Route::Title);
    }

    None
}
