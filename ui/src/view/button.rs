use crate::view::element;
use crate::view::element::Element;

////////////////////////////////////////////////////////////////
// Types //
////////////////////////////////////////////////////////////////

pub struct Button<MSG> {
    label: String,
    on_click: MSG,
}

////////////////////////////////////////////////////////////////
// API //
////////////////////////////////////////////////////////////////

pub fn button<MSG>(label: &str, on_click: MSG) -> Button<MSG> {
    Button {
        label: label.to_string(),
        on_click,
    }
}

impl<T> Button<T> {
    pub fn view(self) -> Element<T> {
        let mut attrs = Vec::new();

        attrs.push(element::on_click(self.on_click));

        element::tag("button", attrs, vec![element::text(self.label.as_str())])
    }
}
