use yew::prelude::Html;
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

pub enum Attribute<MSG> {
    OnClick(MSG),
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

impl<T> Element<T> {
    pub fn to_html<U>(&self, to_msg: fn(T) -> U) -> Html {
        match self {
            Element::Text(text_content) => {
                virtual_dom::VNode::VText(VText::new(text_content.to_string()))
            }
            Element::Node {
                tag,
                attrs: _,
                children,
            } => {
                let mut tag: VTag = VTag::new(tag.to_string());

                for child in children {
                    tag.children.push(child.to_html(to_msg));
                }

                virtual_dom::VNode::VTag(Box::new(tag))
            }
        }
    }
}
