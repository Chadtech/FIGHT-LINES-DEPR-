use crate::session::Session;
use crate::view::grid::cell::{cell, Cell};
use crate::view::grid::row;
use crate::view::grid::row::{row, Row};
use crate::view::image;
use crate::view::sprite::{grass_tile, light_tank};
use crate::view::text::text;
use crate::web_sys::HtmlCanvasElement;
use seed::{prelude::*, *};
use std::f64;

////////////////////////////////////////////////////////////////
// TYPES //
////////////////////////////////////////////////////////////////

pub struct Model {
    session: Session,
    canvas: ElRef<HtmlCanvasElement>,
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
        canvas: ElRef::default(),
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
    let session = &model.session;

    vec![
        canvas_view(&model.canvas),
        text(session.get_fps_str().as_str()),
    ]
}

fn canvas_view(canvas_ref: &ElRef<HtmlCanvasElement>) -> Node<Msg> {
    match canvas_ref.get() {
        None => {}
        Some(canvas) => {
            let ctx = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .unwrap();

            ctx.begin_path();

            // Draw the outer circle.
            ctx.arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
                .unwrap();

            ctx.stroke();
        }
    }

    canvas![el_ref(canvas_ref)]
}

const SIZE: u8 = 5;
