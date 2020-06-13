#![allow(clippy::wildcard_imports)]
use crate::route::Route;
use crate::session::Session;
use seed::{prelude::*, *};

mod page;
mod route;
mod session;
mod view;

////////////////////////////////////////////////////////////////
// TYPES //
////////////////////////////////////////////////////////////////

enum Model {
    PageNotFound(Session),
    Title(Session),
    StartGame(page::start_game::Model),
    Demo(Session, page::demo::Form),
    Lobby(page::lobby::Model),
    Game(page::game::Model),
}

enum Msg {
    RouteChanged(Option<Route>),
    StartGame(page::start_game::Msg),
    Demo(page::demo::Msg),
    Lobby(page::lobby::Msg),
    Game(page::game::Msg),
}
////////////////////////////////////////////////////////////////
// PRIVATE HELPERS //
////////////////////////////////////////////////////////////////

impl Model {
    pub fn get_session(&mut self) -> Session {
        match &self {
            Model::PageNotFound(session) => *session,
            Model::Title(session) => *session,
            Model::StartGame(sub_model) => sub_model.get_session(),
            Model::Demo(session, _) => *session,
            Model::Lobby(sub_model) => sub_model.get_session(),
            Model::Game(sub_model) => sub_model.get_session(),
        }
    }
}

////////////////////////////////////////////////////////////////
// INIT //
////////////////////////////////////////////////////////////////

fn before_mount(_: Url) -> BeforeMount {
    BeforeMount::new().mount_type(MountType::Takeover)
}

fn after_mount(url: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    orders.send_msg(Msg::RouteChanged(route::parse(url)));

    // TODO we need some kind of logic to determine
    // if we should use `init_dev()`, because in some
    // cases, like during a real deployment, we dont want a
    // dev session
    let model = Model::PageNotFound(session::init_dev());
    AfterMount::new(model).url_handling(UrlHandling::PassToRoutes)
}

////////////////////////////////////////////////////////////////
// UPDATE //
////////////////////////////////////////////////////////////////

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::RouteChanged(maybe_route) => {
            handle_route(maybe_route, model);
        }
        Msg::StartGame(sub_msg) => {
            if let Model::StartGame(sub_model) = model {
                page::start_game::update(sub_msg, sub_model, &mut orders.proxy(Msg::StartGame))
            }
        }
        Msg::Demo(sub_msg) => {
            if let Model::Demo(_, sub_model) = model {
                page::demo::update(sub_msg, sub_model)
            }
        }
        Msg::Lobby(sub_msg) => {
            if let Model::Lobby(sub_model) = model {
                page::lobby::update(sub_msg, sub_model)
            }
        }
        Msg::Game(sub_msg) => {
            if let Model::Game(sub_model) = model {
                page::game::update(sub_msg, sub_model)
            }
        }
    }
}

fn handle_route(maybe_route: Option<Route>, model: &mut Model) {
    let session = model.get_session();
    match maybe_route {
        None => *model = Model::PageNotFound(session),
        Some(route) => match route {
            Route::Title => match model {
                Model::Title(_) => {}
                _ => *model = Model::Title(session),
            },
            Route::StartGame => match model {
                Model::StartGame(_) => {}
                _ => *model = Model::StartGame(page::start_game::init(session)),
            },
            Route::Demo => match model {
                Model::Demo(_, _) => {}
                _ => *model = Model::Demo(session, page::demo::init()),
            },
            Route::Lobby(game_id) => match model {
                Model::Lobby(sub_model) => {
                    if sub_model.get_game_id() != game_id {
                        *model = Model::Lobby(page::lobby::init(session, game_id))
                    }
                }
                _ => *model = Model::Lobby(page::lobby::init(session, game_id)),
            },
            Route::Game(game_id) => match model {
                Model::Game(sub_model) => {
                    if sub_model.get_game_id() != game_id {
                        *model = Model::Game(page::game::init(session, game_id))
                    }
                }
                _ => *model = Model::Game(page::game::init(session, game_id)),
            },
        },
    }
}

////////////////////////////////////////////////////////////////
// VIEW //
////////////////////////////////////////////////////////////////

fn view(model: &Model) -> Node<Msg> {
    let body: Vec<Node<Msg>> = match model {
        Model::Title(_) => page::title::view(),
        Model::PageNotFound(_) => vec![div!["Page not found!"]],
        Model::StartGame(sub_model) => page::start_game::view(sub_model)
            .into_iter()
            .map(|node| node.map_msg(Msg::StartGame))
            .collect(),
        Model::Demo(_, sub_model) => page::demo::view(sub_model)
            .into_iter()
            .map(|node| node.map_msg(Msg::Demo))
            .collect(),
        Model::Lobby(sub_model) => page::lobby::view(sub_model)
            .into_iter()
            .map(|node| node.map_msg(Msg::Lobby))
            .collect(),
        Model::Game(sub_model) => page::game::view(sub_model)
            .into_iter()
            .map(|node| node.map_msg(Msg::Game))
            .collect(),
    };

    div![nodes![body]]
}

////////////////////////////////////////////////////////////////
// VIEW //
////////////////////////////////////////////////////////////////

#[wasm_bindgen(start)]
pub fn start() {
    App::builder(update, view)
        .before_mount(before_mount)
        .after_mount(after_mount)
        .routes(|url| Some(Msg::RouteChanged(route::parse(url))))
        .build_and_start();
}
