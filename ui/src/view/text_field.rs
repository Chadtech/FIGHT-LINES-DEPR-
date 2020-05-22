use seed::dom_entity_names::Tag;
use seed::log;
use seed::prelude::*;
use std::borrow::Cow;

////////////////////////////////////////////////////////////////
// Types //
////////////////////////////////////////////////////////////////

#[derive(Copy, Clone)]
pub struct TextField<'a> {
    value: &'a str,
    placeholder: Option<&'a str>,
}

////////////////////////////////////////////////////////////////
// API //
////////////////////////////////////////////////////////////////

pub fn text_field<'a>(value: &'a str) -> TextField<'a> {
    TextField {
        value,
        placeholder: None,
    }
}

impl<'a> TextField<'a> {
    pub fn view<T>(self) -> Node<T> {
        let mut element: El<T> = El::empty(Tag::Custom(Cow::Borrowed("input")));

        element.add_event_handler(ev(Ev::Input, |new_value| {
            log(new_value);

            ()
        }));

        element.add_attr(Cow::Borrowed("value"), self.value);

        match self.placeholder {
            None => {}
            Some(placeholder_text) => {
                element.add_attr(Cow::Borrowed("placeholder"), placeholder_text);
            }
        }

        Node::Element(element)
    }

    pub fn placeholder(&'a mut self, placeholder_text: &'a str) -> &'a mut Self {
        self.placeholder = Some(placeholder_text);
        self
    }
}
