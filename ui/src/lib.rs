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
}

#[derive(Clone)]
enum Msg {
    RouteChanged(Option<Route>),
    StartGameMsg(page::start_game::Msg),
    DemoMsg(page::demo::Msg),
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
        }
    }
}

////////////////////////////////////////////////////////////////
// INIT //
////////////////////////////////////////////////////////////////

fn before_mount(_: Url) -> BeforeMount {
    BeforeMount::new().mount_type(MountType::Takeover)
}

fn after_mount<'a>(url: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
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
        Msg::StartGameMsg(sub_msg) => {
            if let Model::StartGame(sub_model) = model {
                page::start_game::update(sub_msg, sub_model, &mut orders.proxy(Msg::StartGameMsg))
            }
        }
        Msg::DemoMsg(sub_msg) => {
            if let Model::Demo(_, sub_model) = model {
                page::demo::update(sub_msg, sub_model)
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
            .map(|node| node.map_msg(Msg::StartGameMsg))
            .collect(),
        Model::Demo(_, sub_model) => page::demo::view(sub_model)
            .into_iter()
            .map(|node| node.map_msg(Msg::DemoMsg))
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
