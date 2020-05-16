use crate::page::title::Msg::StartClicked;
use crate::view::button::button;
use crate::view::element::{text, Element};
use crate::view::grid::cell::cell;
use crate::view::grid::row;
use crate::view::grid::row::row;

////////////////////////////////////////////////////////////////
// Types  //
////////////////////////////////////////////////////////////////

#[derive(Clone, Copy)]
pub struct Model {}

#[derive(Clone, Copy)]
pub enum Msg {
    StartClicked,
}

////////////////////////////////////////////////////////////////
// Init //
////////////////////////////////////////////////////////////////

pub fn init() -> Model {
    Model {}
}

////////////////////////////////////////////////////////////////
// View //
////////////////////////////////////////////////////////////////

pub fn view(_model: Model) -> Vec<Element<Msg>> {
    row::many(vec![
        row(vec![
            cell(vec![text("FIGHT LINES")]),
            cell(vec![button("Press", StartClicked).view()]),
        ])
        .center(),
        row(vec![cell(vec![text("THE GAME")])]).center(),
    ])
    .view()
}
