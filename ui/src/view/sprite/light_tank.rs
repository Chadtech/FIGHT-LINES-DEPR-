use crate::session::Session;
use crate::view::image;
use crate::view::image::Image;
use std::num;

////////////////////////////////////////////////////////////////
// Types //
////////////////////////////////////////////////////////////////

pub enum Model {
    FireCannon,
}

static FIRE_CANNON_ANIMATION_LENGTH: i64 = 15;

////////////////////////////////////////////////////////////////
// CONSTS //
////////////////////////////////////////////////////////////////

static FILE_NAME: &str = "light_tank";

////////////////////////////////////////////////////////////////
// API //
////////////////////////////////////////////////////////////////

impl Model {
    pub fn init() -> Model {
        Model::FireCannon
    }

    pub fn animation_length(&self) -> i64 {
        match self {
            Model::FireCannon => FIRE_CANNON_ANIMATION_LENGTH,
        }
    }

    pub fn sprite_sheet_offset(&self, step: i64) -> image::Position {
        match self {
            Model::FireCannon => image::Position {
                x: step * (-96 as i64),
                y: 192,
            },
        }
    }

    pub fn to_img(&self, session: &Session) -> Image {
        let animation_step = session.get_frame() % (self.animation_length());

        let offset = self.sprite_sheet_offset(animation_step);
        Image::from_source(session.asset_url(FILE_NAME))
            .with_size(image::Size {
                width: 96,
                height: 96,
            })
            .with_source_offset(offset)
            .at_position(image::Position { x: -24, y: -24 })
    }
}
