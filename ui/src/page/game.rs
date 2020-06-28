use crate::session::Session;
use crate::view::grid::cell::{cell, Cell};
use crate::view::grid::row;
use crate::view::grid::row::{row, Row};
use crate::view::image;
use crate::view::sprite::{grass_tile, light_tank};
use crate::view::text::text;
use seed::prelude::Node;

////////////////////////////////////////////////////////////////
// TYPES //
////////////////////////////////////////////////////////////////

pub struct Model {
    session: Session,
    game_id: String,
    tank_position: Position,
}
#[derive(Clone)]
pub struct Position {
    x: u8,
    y: u8,
}

pub enum Msg {
    CellClicked(Position),
}

impl Model {
    pub fn get_session(&self) -> Session {
        self.session
    }

    pub fn get_session_mut(&mut self) -> &mut Session {
        &mut self.session
    }

    pub fn get_game_id(&self) -> String {
        self.game_id.clone()
    }

    pub fn set_tank_position(&mut self, position: Position) {
        self.tank_position = position;
    }
}
////////////////////////////////////////////////////////////////
// INIT //
////////////////////////////////////////////////////////////////

pub fn init(session: Session, game_id: String) -> Model {
    Model {
        session,
        game_id,
        tank_position: Position { x: 0, y: 0 },
    }
}

////////////////////////////////////////////////////////////////
// UPDATE //
////////////////////////////////////////////////////////////////

pub fn update(msg: Msg, model: &mut Model) {
    match msg {
        Msg::CellClicked(position) => {
            model.set_tank_position(position);
        }
    }
}

////////////////////////////////////////////////////////////////
// VIEW //
////////////////////////////////////////////////////////////////

pub fn view(model: &Model) -> Vec<Node<Msg>> {
    let session = model.get_session();

    let mut grass_tile_rows: Vec<Row<Msg>> = vec![];

    for x in 0..SIZE {
        let mut grass_tile_cells: Vec<Cell<Msg>> = vec![];

        for y in 0..SIZE {
            let tank_position = &model.tank_position;

            let tank = if x == tank_position.x && y == tank_position.y {
                light_tank::Model::init().to_img(session).view()
            } else {
                text("")
            };

            grass_tile_cells.push(
                cell(vec![grass_tile::Model::init().to_img(session).view(), tank])
                    .on_click(move |_| Msg::CellClicked(Position { x, y })),
            )
        }

        grass_tile_rows.push(row(grass_tile_cells));
    }
    let timestamp_str = session.get_render_delta().to_string();

    let timestamp_row = row::single(text(timestamp_str.as_str()));

    grass_tile_rows.push(timestamp_row);
    row::many(grass_tile_rows).view()
}

const SIZE: u8 = 20;
