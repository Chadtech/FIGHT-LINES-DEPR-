use crate::route::Route;
use crate::view::button;
use crate::view::grid::row;
use crate::view::text::text;
use seed::prelude::*;

////////////////////////////////////////////////////////////////
// VIEW //
////////////////////////////////////////////////////////////////

pub fn view<MSG>() -> Vec<Node<MSG>>
where
    MSG: Clone,
    MSG: 'static,
{
    row::many(vec![
        row::single(text("FIGHT LINES")),
        row::single(button::route("Start", Route::StartGame).view()),
    ])
    .map_rows(|row| row.center(true))
    .view()
}
