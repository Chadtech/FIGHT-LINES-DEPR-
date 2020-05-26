use crate::route::Route;
use crate::view::grid::cell;

use crate::view::button;
use crate::view::grid::row;
use crate::view::text::text;
use seed::{prelude::*, *};

////////////////////////////////////////////////////////////////
// TYPES //
////////////////////////////////////////////////////////////////

pub struct Model {
    clicked: bool,
}

#[derive(Copy, Clone)]
pub enum Msg {
    StartClicked,
}

////////////////////////////////////////////////////////////////
// INIT //
////////////////////////////////////////////////////////////////

pub fn init() -> Model {
    Model { clicked: false }
}

////////////////////////////////////////////////////////////////
// UPDATE //
////////////////////////////////////////////////////////////////

pub fn update(msg: Msg, model: &mut Model) {
    match msg {
        Msg::StartClicked => {
            println!("World at work");
            model.clicked = true;
        }
    }
}

////////////////////////////////////////////////////////////////
// VIEW //
////////////////////////////////////////////////////////////////

pub fn view(model: &Model) -> Vec<Node<Msg>> {
    row::many(vec![
        row::single(text("Demo Page")),
        row::single(button::button("Demo Start Button", |_| Msg::StartClicked).view()),
        row::single(go_view(model.clicked)),
        row::single(h1!["The Grand Total"]),
        row::row(vec![
            cell::single(label!["Game Name: "]),
            cell::single(input![attrs! {
                At::Placeholder => "Name of Game"
            }]),
        ]),
        row::row(vec![
            cell::single(label!["Number Players: ",]),
            cell::single(input![attrs! {
                At::Placeholder => "3"
                At::Type => "number"
            }]),
        ]),
        row::single(button::route("Submit", Route::StartGame).view()),
    ])
    .map_rows(|row| row.center(true))
    .view()
}

fn go_view(go: bool) -> Node<Msg> {
    let content = if go {
        "YOU ARE NOW PLAYING FIGHT LINES!!!"
    } else {
        ""
    };

    text(content)
}
