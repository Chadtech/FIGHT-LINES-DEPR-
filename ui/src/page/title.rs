use crate::view::button::button;
use crate::view::grid::cell::cell;
use crate::view::grid::row;
use crate::view::grid::row::row;
use seed::div;
use seed::prelude::*;

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
            model.clicked = true;
        }
    }
}

////////////////////////////////////////////////////////////////
// VIEW //
////////////////////////////////////////////////////////////////

pub fn view(model: &Model) -> Vec<Node<Msg>> {
    row::many(vec![
        row(vec![cell(vec![div!["FIGHT LINES"]])]).center(true),
        row(vec![cell(vec![button("Start", Msg::StartClicked).view()])]).center(true),
        row(vec![cell(vec![go_view(model.clicked)])]).center(true),
    ])
    .view()
}

fn go_view(go: bool) -> Node<Msg> {
    let text = if go {
        "YOU ARE NOW PLAYING FIGHT LINES!!!"
    } else {
        ""
    };

    div![text]
}
