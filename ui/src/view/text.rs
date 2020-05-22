use seed::prelude::*;
use seed::virtual_dom::Text;

pub fn text<MSG>(str: &str) -> Node<MSG> {
    let node_text: Text = Text::new(str.to_string());
    Node::Text(node_text)
}
