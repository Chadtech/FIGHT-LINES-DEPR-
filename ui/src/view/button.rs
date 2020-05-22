use crate::route;
use crate::view::button::Click::{Handler, Route};
use crate::view::text::text;
use seed::dom_entity_names::Tag;
use seed::prelude::*;
use seed::prelude::{El, Node};
use std::borrow::Cow;
use std::rc::Rc;

////////////////////////////////////////////////////////////////
// Types //
////////////////////////////////////////////////////////////////

pub struct Button<MSG> {
    label: String,
    on_click: Click<MSG>,
}

enum Click<MSG> {
    Handler(MSG),
    Route(route::Route),
}

////////////////////////////////////////////////////////////////
// API //
////////////////////////////////////////////////////////////////

pub fn button<MSG>(label: String, on_click: MSG) -> Button<MSG> {
    Button {
        label: label,
        on_click: Handler(on_click),
    }
}

pub fn route<MSG>(label: &str, route: route::Route) -> Button<MSG> {
    Button {
        label: label.to_string(),
        on_click: Route(route),
    }
}

impl<T> Button<T> {
    pub fn view(self) -> Node<T> {
        let tag = match self.on_click {
            Handler(_) => "button",
            Route(_) => "a",
        };

        let mut element: El<T> = El::empty(Tag::Custom(Cow::Borrowed(tag)));

        element.children.push(text(self.label.as_str()));

        match self.on_click {
            Handler(msg) => {
                // element.add_event_handler(ev(Ev::Click, |_| msg));
                element.add_event_handler(EventHandler {
                    trigger: Ev::Click,
                    callback: Rc::new(|_| Some(msg)),
                });
            }
            Route(route) => {
                element.add_attr(Cow::Borrowed("href"), route.to_string());
            }
        }

        Node::Element(element)
    }
}
