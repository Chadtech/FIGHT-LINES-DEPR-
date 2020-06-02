use seed::dom_entity_names::Tag;
use seed::prelude::*;
use std::borrow::Cow;
use std::rc::Rc;

////////////////////////////////////////////////////////////////
// Types //
////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct TextField<MSG: 'static> {
    value: String,
    placeholder: Option<String>,
    on_input: Rc<dyn Fn(String) -> MSG>,
}

////////////////////////////////////////////////////////////////
// API //
////////////////////////////////////////////////////////////////

pub fn text_field<MSG>(
    value: &str,
    on_input: impl FnOnce(String) -> MSG + Clone + 'static,
) -> TextField<MSG> {
    TextField {
        value: value.to_string(),
        placeholder: None,
        on_input: Rc::new(move |event| on_input.clone()(event)),
    }
}

impl<T> TextField<T> {
    pub fn view(self) -> Node<T>
    where
        T: Clone,
        T: 'static,
    {
        let mut element: El<T> = El::empty(Tag::Custom(Cow::Borrowed("input")));

        let on_input = self.on_input;

        element.add_event_handler(input_ev(Ev::Input, move |event| on_input(event)));

        element.add_attr(Cow::Borrowed("value"), self.value);

        match self.placeholder {
            None => {}
            Some(placeholder_text) => {
                element.add_attr(Cow::Borrowed("placeholder"), placeholder_text);
            }
        }

        Node::Element(element)
    }

    pub fn placeholder(mut self, placeholder_text: &str) -> TextField<T> {
        self.placeholder = Some(placeholder_text.to_string());
        self
    }
}
