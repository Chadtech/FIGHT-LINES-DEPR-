use crate::program;
use crate::program::Program;
use std::rc::Rc;
use yew::callback::Callback;
use yew::html;
use yew::prelude::{Component, ComponentLink, Html};
use yew::services::console::ConsoleService;
use yew::virtual_dom;
use yew::virtual_dom::vtag::VTag;
use yew::virtual_dom::vtext::VText;

////////////////////////////////////////////////////////////////
// TYPES //
////////////////////////////////////////////////////////////////

pub enum Element<MSG> {
    Text(String),
    Node {
        tag: String,
        attrs: Vec<Attribute<MSG>>,
        children: Vec<Element<MSG>>,
    },
}

#[derive(Clone, Copy)]
pub enum Attribute<MSG> {
    OnClick(MSG),
    Center,
}

////////////////////////////////////////////////////////////////
// API //
////////////////////////////////////////////////////////////////

pub fn text<MSG>(str: &str) -> Element<MSG> {
    Element::Text(str.to_string())
}

pub fn tag<MSG>(
    tag_name: &str,
    attrs: Vec<Attribute<MSG>>,
    children: Vec<Element<MSG>>,
) -> Element<MSG> {
    Element::Node {
        tag: tag_name.to_string(),
        attrs,
        children,
    }
}

static NODE_TAG_NAME: &'static str = "box";

pub fn node<MSG>(children: Vec<Element<MSG>>) -> Element<MSG> {
    tag(NODE_TAG_NAME, Vec::new(), children)
}

pub fn center<MSG>() -> Attribute<MSG> {
    use crate::view::element::Attribute::Center;
    Center
}

pub fn on_click<MSG>(msg: MSG) -> Attribute<MSG> {
    use crate::view::element::Attribute::OnClick;
    OnClick(msg)
}

impl<T> Element<T>
where
    T: Clone,
{
    pub fn html(self, link: &ComponentLink<Program>, to_msg: fn(T) -> program::Msg) -> Html {
        match self {
            Element::Text(text_content) => {
                virtual_dom::VNode::VText(VText::new(text_content.to_string()))
            }
            Element::Node {
                tag,
                attrs,
                children,
            } => {
                let mut tag: VTag = VTag::new(tag.to_string());

                for attr in attrs {
                    match attr {
                        Attribute::OnClick(msg) => {
                            let wrapped_msg = to_msg(msg);
                            let send = |_| {
                                link.send_message(wrapped_msg);
                            };

                            let send_rc = Rc::new(send);

                            let callback = Callback::Callback(send_rc);

                            let on_click_listener = html::onclick::Wrapper::new(callback);

                            tag.listeners.push(Rc::new(on_click_listener));
                        }
                        Attribute::Center => tag.classes.push("center"),
                    }
                }

                for child in children {
                    tag.children.push(child.html(link, to_msg));
                }

                virtual_dom::VNode::VTag(Box::new(tag))
            }
        }
    }
    pub fn map<U>(self, mapper: fn(T) -> U) -> Element<U> {
        match self {
            Element::Text(text_content) => Element::Text(text_content.to_string()),
            Element::Node {
                tag,
                attrs,
                children,
            } => Element::Node {
                tag: tag.to_string(),
                attrs: attrs.into_iter().map(|attr| attr.map(mapper)).collect(),
                children: children
                    .into_iter()
                    .map(|child| child.map(mapper))
                    .collect(),
            },
        }
    }
}

impl<T> Attribute<T> {
    pub fn map<U>(self, mapper: fn(T) -> U) -> Attribute<U> {
        match self {
            Attribute::OnClick(msg) => Attribute::OnClick(mapper(msg)),
            Attribute::Center => Attribute::Center,
        }
    }
}
