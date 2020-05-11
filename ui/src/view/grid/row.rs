use crate::view::element;
use crate::view::element::{tag, Element};
use crate::view::grid::cell::Cell;

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

impl<T> Row<T> {
    pub fn view(self) -> Element<T> {
        let element_children: Vec<Element<T>> =
            self.cells.into_iter().map(|cell| cell.view()).collect();

        let mut attrs = Vec::new();

        if self.center {
            attrs.push(element::center());
        }

        tag("row", attrs, element_children)
    }

    pub fn center(mut self) -> Row<T> {
        self.center = true;
        self
    }
}

pub fn many<MSG>(rows: Vec<Row<MSG>>) -> Many<MSG> {
    Many { rows }
}

impl<T> Many<T> {
    pub fn view(self) -> Vec<Element<T>> {
        let iter_rows = self.rows.into_iter();

        iter_rows.map(|row| row.view()).collect()
    }
}
