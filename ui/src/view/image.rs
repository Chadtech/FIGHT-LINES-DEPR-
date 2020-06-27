use seed::dom_entity_names::Tag;
use seed::prelude::{El, Node};
use std::borrow::Cow;

////////////////////////////////////////////////////////////////
// Types //
////////////////////////////////////////////////////////////////

pub struct Image {
    src: String,
    src_offset: Option<Position>,
    position: Option<Position>,
    size: Option<Size>,
}

pub struct Position {
    pub x: i64,
    pub y: i64,
}

pub struct Size {
    pub width: u8,
    pub height: u8,
}

impl Image {
    pub fn from_source(src: String) -> Image {
        Image {
            src,
            position: None,
            src_offset: None,
            size: None,
        }
    }

    pub fn with_source_offset(mut self, position: Position) -> Image {
        self.src_offset = Some(position);
        self
    }

    pub fn at_position(mut self, position: Position) -> Image {
        self.position = Some(position);
        self
    }

    pub fn with_size(mut self, size: Size) -> Image {
        self.size = Some(size);
        self
    }

    pub fn view<T>(self) -> Node<T> {
        let mut element: El<T> = El::empty(Tag::Custom(Cow::Borrowed("div")));

        if let Some(position) = self.position {
            let mut x_buf: String = position.x.to_string();
            x_buf.push_str("px");

            let mut y_buf: String = position.x.to_string();
            y_buf.push_str("px");

            element.add_style("position", "absolute");
            element.add_style("top", y_buf);
            element.add_style("left", x_buf);
        }

        if let Some(size) = self.size {
            let mut width_buf: String = size.width.to_string();
            width_buf.push_str("px");

            let mut height_buf: String = size.height.to_string();
            height_buf.push_str("px");

            element.add_style("width", width_buf);
            element.add_style("height", height_buf);
        }

        element.add_class(Cow::Borrowed("sprite"));

        let mut src_buf = String::new();
        src_buf.push_str("url(");
        src_buf.push_str(self.src.as_str());
        src_buf.push_str(")");

        if let Some(src_offset) = self.src_offset {
            src_buf.push_str(" ");
            src_buf.push_str(src_offset.x.to_string().as_str());
            src_buf.push_str("px ");
            src_buf.push_str(src_offset.y.to_string().as_str());
            src_buf.push_str("px");
        }

        element.add_style("background", src_buf);

        Node::Element(element)
    }
}
