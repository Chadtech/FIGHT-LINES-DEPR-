use crate::route;
use crate::view::button::Click::{Handler, Route};
use crate::view::text::text;
use seed::dom_entity_names::Tag;
use seed::prelude::*;
use seed::prelude::{El, Node};
use std::borrow::Cow;

////////////////////////////////////////////////////////////////
// Types //
////////////////////////////////////////////////////////////////

#[derive(Copy, Clone)]
pub struct Button<'a, MSG> {
    label: &'a str,
    on_click: Click<MSG>,
}

#[derive(Copy, Clone)]
enum Click<MSG> {
    Handler(MSG),
    Route(route::Route),
}

////////////////////////////////////////////////////////////////
// API //
////////////////////////////////////////////////////////////////

pub fn button<'a, MSG>(label: &'a str, on_click: MSG) -> Button<'a, MSG> {
    Button {
        label: label,
        on_click: Handler(on_click),
    }
}

pub fn route<'a, MSG>(label: &'a str, route: route::Route) -> Button<'a, MSG> {
    Button {
        label: label,
        on_click: Route(route),
    }
}

impl<T> Button<'static, T>
where
    T: 'static,
    T: Clone,
{
    pub fn view(self) -> Node<T> {
        let tag = match self.on_click {
            Handler(_) => "button",
            Route(_) => "a",
        };

        let mut element: El<T> = El::empty(Tag::Custom(Cow::Borrowed(tag)));

        element.children.push(text(self.label));

        match self.on_click {
            Handler(msg) => {
                element.add_event_handler(ev(Ev::Click, |_| msg));
            }
            Route(route) => {
                element.add_attr(Cow::Borrowed("href"), route.as_str());
            }
        }

        Node::Element(element)
    }
}
