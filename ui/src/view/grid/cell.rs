use seed::dom_entity_names::Tag;
use seed::prelude::*;
use std::borrow::Cow;
use std::rc::Rc;
use web_sys::MouseEvent;

////////////////////////////////////////////////////////////////
// Types //
////////////////////////////////////////////////////////////////

pub struct Cell<MSG> {
    children: Vec<Node<MSG>>,
    on_click: Option<(Rc<dyn Fn(MouseEvent) -> MSG>)>,
}

////////////////////////////////////////////////////////////////
// API //
////////////////////////////////////////////////////////////////

pub fn cell<MSG>(children: Vec<Node<MSG>>) -> Cell<MSG> {
    Cell {
        children,
        on_click: None,
    }
}

pub fn single<MSG>(child: Node<MSG>) -> Cell<MSG> {
    cell(vec![child])
}

impl<T> Cell<T>
where
    T: 'static,
{
    pub fn view(self) -> Node<T> {
        let mut element: El<T> = El::empty(Tag::Custom(Cow::Borrowed("cell")));
        element.children = self.children;

        match self.on_click {
            Some(on_click) => {
                element.add_event_handler(mouse_ev(Ev::Click, move |event| on_click(event)));
            }
            None => {}
        }

        Node::Element(element)
    }

    pub fn on_click(mut self, on_click: impl FnOnce(MouseEvent) -> T + Clone + 'static) -> Cell<T> {
        self.on_click = Some(Rc::new(move |event| on_click.clone()(event)));
        self
    }
}
