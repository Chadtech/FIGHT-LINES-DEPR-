use seed::Url;

////////////////////////////////////////////////////////////////
// TYPES //
////////////////////////////////////////////////////////////////

#[derive(Copy, Clone)]
pub enum Route {
    Title,
    StartGame,
    Demo,
}

////////////////////////////////////////////////////////////////
// API //
////////////////////////////////////////////////////////////////

impl Route {
    pub fn as_str(self) -> &'static str {
        match self {
            Route::Title => "",
            Route::StartGame => "start-game",
            Route::Demo => "demo",
        }
    }
}

pub fn parse(url: Url) -> Option<Route> {
    if url.path().is_empty() {
        return Some(Route::Title);
    }
    if url.path()[0].eq("demo") {
        return Some(Route::Demo);
    }

    None
}
