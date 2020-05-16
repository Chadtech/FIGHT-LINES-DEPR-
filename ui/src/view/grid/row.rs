use crate::view::grid::cell::Cell;
use seed::dom_entity_names::Tag;
use seed::prelude::*;
use std::borrow::Cow;

////////////////////////////////////////////////////////////////
// Types //
////////////////////////////////////////////////////////////////

pub struct Row<MSG> {
    cells: Vec<Cell<MSG>>,
    center: bool,
}

pub struct Many<MSG> {
    rows: Vec<Row<MSG>>,
}

////////////////////////////////////////////////////////////////
// API //
////////////////////////////////////////////////////////////////

pub fn row<MSG>(cells: Vec<Cell<MSG>>) -> Row<MSG> {
    Row {
        cells,
        center: false,
    }
}

pub fn many<MSG>(rows: Vec<Row<MSG>>) -> Many<MSG> {
    Many { rows }
}

impl<T> Row<T> {
    pub fn view(self) -> Node<T> {
        let mut element: El<T> = El::empty(Tag::Custom(Cow::Borrowed("row")));

        if self.center {
            element.add_class("center");
        }

        element.children = self.cells.into_iter().map(|cell| cell.view()).collect();
        Node::Element(element)
    }

    pub fn center(mut self, should_center: bool) -> Row<T> {
        self.center = should_center;
        self
    }
}

impl<T> Many<T> {
    pub fn view(self) -> Vec<Node<T>> {
        let iter_rows = self.rows.into_iter();

        iter_rows.map(|row| row.view()).collect()
    }
}
