use crate::view::style::gen_const::{padding_bottom, padding_left, padding_right, padding_top};
use seed::prelude::*;

/// This module name comes from generated code, so if
/// you rename it here, also rename it in the script
/// which is located in /code-gen/src/main.rs
#[allow(dead_code)]
pub mod gen_const;

pub struct Style {
    classes: Vec<&'static str>,
}

pub fn apply<'a, T>(styles: Vec<Style>, element: &'a mut El<T>) {
    let classes: Vec<&'static str> = styles
        .into_iter()
        .map(|style| style.classes)
        .flatten()
        .collect();

    for class in classes {
        &element.add_class(class);
    }
}

pub fn padding(v: u8) -> Style {
    padding_each(v, v, v, v)
}

pub fn padding_each(top: u8, right: u8, bottom: u8, left: u8) -> Style {
    Style {
        classes: vec![
            padding_top(top),
            padding_right(right),
            padding_bottom(bottom),
            padding_left(left),
        ],
    }
}
