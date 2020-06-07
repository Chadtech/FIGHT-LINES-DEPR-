use crate::session::Session;
use seed::dom_entity_names::Tag;
use seed::prelude::{El, Node};
use std::borrow::Cow;

////////////////////////////////////////////////////////////////
// Types //
////////////////////////////////////////////////////////////////

pub struct Sprite {
    src: Source,
    position: Option<Position>,
}

pub struct Position {
    pub x: i16,
    pub y: i16,
}

pub enum Source {
    GrassTile,
    LightTank,
}

pub static GRASS_TILE_FILE_NAME: &'static str = "grass_square";
pub static LIGHT_TANK_FILE_NAME: &'static str = "light_tank";

impl Source {
    pub fn file_name(self) -> &'static str {
        match self {
            Source::GrassTile => GRASS_TILE_FILE_NAME,
            Source::LightTank => LIGHT_TANK_FILE_NAME,
        }
    }
}

impl Sprite {
    pub fn from_source(src: Source) -> Sprite {
        let default_position: Option<Position> = None;
        Sprite {
            src,
            position: default_position,
        }
    }
    pub fn at_position(mut self, position: Position) -> Sprite {
        self.position = Some(position);
        self
    }
    pub fn view<T>(self, session: Session) -> Node<T> {
        let mut element: El<T> = El::empty(Tag::Custom(Cow::Borrowed("img")));

        let mut url = String::new();
        url.push_str("/assets/");
        url.push_str(self.src.file_name());
        url.push_str(".png");

        match self.position {
            Some(position) => {
                element.add_style("position", "absolute");
            }
            None => {}
        }

        element.add_class(Cow::Borrowed("sprite"));
        element.add_attr(Cow::Borrowed("src"), session.url(url.as_str()));

        Node::Element(element)
    }
}
