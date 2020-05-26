#![allow(clippy::wildcard_imports)]
use crate::route::Route;
use seed::{prelude::*, *};

mod page;
mod route;
mod view;

////////////////////////////////////////////////////////////////
// TYPES //
////////////////////////////////////////////////////////////////

enum Model {
    PageNotFound,
    Title,
    StartGame(page::start_game::Model),
}

impl Default for Model {
    fn default() -> Self {
        Model::PageNotFound
    }
}

#[derive(Clone)]
enum Msg {
    RouteChanged(Option<Route>),
    StartGameMsg(page::start_game::Msg),
}

////////////////////////////////////////////////////////////////
// INIT //
////////////////////////////////////////////////////////////////

fn before_mount(_: Url) -> BeforeMount {
    // Since we have the "loading..." text in the app section of index.html,
    // we use MountType::Takover which will overwrite it with the seed generated html
    BeforeMount::new().mount_type(MountType::Takeover)
}

fn after_mount<'a>(url: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    orders.send_msg(Msg::RouteChanged(route::parse(url)));

    let model = Model::PageNotFound;
    AfterMount::new(model).url_handling(UrlHandling::PassToRoutes)
}

////////////////////////////////////////////////////////////////
// UPDATE //
////////////////////////////////////////////////////////////////

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::RouteChanged(maybe_route) => {
            handle_route(maybe_route, model);
        }
        Msg::StartGameMsg(sub_msg) => {
            if let Model::StartGame(sub_model) = model {
                page::start_game::update(sub_msg, sub_model)
            }
        }
    }
}

fn handle_route(maybe_route: Option<Route>, model: &mut Model) {
    match maybe_route {
        None => *model = Model::PageNotFound,
        Some(route) => match route {
            Route::Title => match model {
                Model::Title => {}
                _ => *model = Model::Title,
            },
            Route::StartGame => match model {
                Model::StartGame(_) => {}
                _ => *model = Model::StartGame(page::start_game::init()),
            },
        },
    }
}

////////////////////////////////////////////////////////////////
// VIEW //
////////////////////////////////////////////////////////////////

fn view(model: &Model) -> Node<Msg> {
    let body: Vec<Node<Msg>> = match model {
        Model::Title => page::title::view(),
        Model::PageNotFound => vec![div!["Page not found!"]],
        Model::StartGame(sub_model) => page::start_game::view(sub_model)
            .into_iter()
            .map(|node| node.map_msg(Msg::StartGameMsg))
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
    // App::start("app", init, update, view);
}
