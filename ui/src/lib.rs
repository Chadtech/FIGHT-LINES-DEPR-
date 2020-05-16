#![allow(clippy::wildcard_imports)]
use crate::route::Route;
use crate::Msg::TitleMsg;
use seed::{prelude::*, *};

mod page;
mod route;
mod view;

////////////////////////////////////////////////////////////////
// TYPES //
////////////////////////////////////////////////////////////////

enum Model {
    PageNotFound,
    Title(page::title::Model),
}

#[derive(Copy, Clone)]
enum Msg {
    TitleMsg(page::title::Msg),
}

////////////////////////////////////////////////////////////////
// INIT //
////////////////////////////////////////////////////////////////

fn init<'a>(url: Url, _: &mut impl Orders<Msg>) -> Model {
    match route::parse(url) {
        None => Model::PageNotFound,
        Some(route) => match route {
            Route::Title => Model::Title(page::title::init()),
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
    }
}

////////////////////////////////////////////////////////////////
// VIEW //
////////////////////////////////////////////////////////////////

fn view(model: &Model) -> Node<Msg> {
    let body: Vec<Node<Msg>> = match model {
        Model::Title(sub_model) => page::title::view(sub_model)
            .into_iter()
            .map(|node| node.map_msg(TitleMsg))
            .collect(),
        Model::PageNotFound => vec![div!["Page not found!"]],
    };

    div![nodes![body]]
}

////////////////////////////////////////////////////////////////
// VIEW //
////////////////////////////////////////////////////////////////

#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
