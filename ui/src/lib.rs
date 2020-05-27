#![allow(clippy::wildcard_imports)]
use crate::route::Route;
use seed::{prelude::*, *};

mod page;
mod requests;
mod route;
mod view;

////////////////////////////////////////////////////////////////
// TYPES //
////////////////////////////////////////////////////////////////

enum Model {
    PageNotFound,
    Title(page::title::Model),
    Demo(page::demo::Form),
}

// impl Default for Model {
//     fn default() -> Self {
//         PageNotFound
//     }
// }

enum Msg {
    RouteChanged(Option<Route>),
    TitleMsg(page::title::Msg),
    DemoMsg(page::demo::Msg),
}

////////////////////////////////////////////////////////////////
// INIT //
////////////////////////////////////////////////////////////////

fn init<'a>(url: Url, _: &mut impl Orders<Msg>) -> Model {
    match route::parse(url) {
        None => Model::PageNotFound,
        Some(route) => match route {
            Route::Title => Model::Title(page::title::init()),
            Route::Demo => Model::Demo(page::demo::init()),
            Route::StartGame => Model::PageNotFound,
        },
    }
}

////////////////////////////////////////////////////////////////
// UPDATE //
////////////////////////////////////////////////////////////////

fn update<'a>(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::TitleMsg(sub_msg) => {
            if let Model::Title(sub_model) = model {
                page::title::update(sub_msg, sub_model)
            }
        }
        Msg::RouteChanged(maybe_route) => {
            handle_route(maybe_route, model);
        }
        Msg::DemoMsg(sub_msg) => {
            if let Model::Demo(sub_model) = model {
                page::demo::update(sub_msg, sub_model)
            }
        }
    }
}

fn handle_route(maybe_route: Option<Route>, model: &mut Model) {
    match maybe_route {
        None => *model = Model::PageNotFound,
        Some(route) => match route {
            Route::Title => match model {
                Model::Title(_) => {}
                _ => *model = Model::Title(page::title::init()),
            },
            Route::Demo => match model {
                Model::Demo(_) => {}
                _ => *model = Model::Demo(page::demo::init()),
            },
            Route::StartGame => *model = Model::PageNotFound,
        },
    }
}

////////////////////////////////////////////////////////////////
// VIEW //
////////////////////////////////////////////////////////////////

fn view(model: &Model) -> Node<Msg> {
    let body: Vec<Node<Msg>> = match model {
        Model::Title(sub_model) => page::title::view(sub_model)
            .into_iter()
            .map(|node| node.map_msg(Msg::TitleMsg))
            .collect(),
        Model::PageNotFound => vec![div!["Page not found!"]],
        Model::Demo(sub_model) => page::demo::view(sub_model)
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
    // App::builder(update, view).build_and_start();
    App::start("app", init, update, view);
}
