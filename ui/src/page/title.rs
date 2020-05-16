use crate::view::button::button;
use crate::view::grid::row;
use crate::view::text::text;
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
        row::single(text("FIGHT LINES")).center(true),
        row::single(button("Start", Msg::StartClicked).view()).center(true),
        row::single(go_view(model.clicked)).center(true),
    ])
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
