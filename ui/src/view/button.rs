use crate::route;
use crate::view::button::Click::{Handler, Route};
use crate::view::text::text;
use seed::dom_entity_names::Tag;
use seed::prelude::*;
use seed::prelude::{El, Node};
use std::borrow::Cow;
use std::rc::Rc;
use web_sys::MouseEvent;

////////////////////////////////////////////////////////////////
// Types //
////////////////////////////////////////////////////////////////

pub struct Button<MSG: 'static> {
    label: String,
    on_click: Click<MSG>,
}

enum Click<MSG> {
    Handler(Rc<dyn Fn(MouseEvent) -> MSG>),
    Route(route::Route),
}

////////////////////////////////////////////////////////////////
// API //
////////////////////////////////////////////////////////////////

pub fn button<MSG>(
    label: &str,
    on_click: impl FnOnce(MouseEvent) -> MSG + Clone + 'static,
) -> Button<MSG> {
    Button {
        label: label.to_string(),
        on_click: Handler(Rc::new(move |event| on_click.clone()(event))),
    }
}

pub fn route<MSG>(label: &str, route: route::Route) -> Button<MSG> {
    Button {
        label: label.to_string(),
        on_click: Route(route),
    }
}

impl<T> Button<T>
where
    T: Clone,
    T: 'static,
{
    pub fn view(self) -> Node<T> {
        let tag = match self.on_click {
            Handler(_) => "button",
            Route(_) => "a",
        };

        let mut element: El<T> = El::empty(Tag::Custom(Cow::Borrowed(tag)));

        element.add_class("button");
        element.children.push(text(self.label.as_str()));

        match self.on_click {
            Handler(on_click) => {
                element.add_event_handler(mouse_ev(Ev::Click, move |event| on_click(event)));
            }
            Route(route) => {
                element.add_attr(Cow::Borrowed("href"), route.to_string());
            }
        }

        Node::Element(element)
    }
}
