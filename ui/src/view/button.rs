use seed::button;
use seed::prelude::*;

////////////////////////////////////////////////////////////////
// Types //
////////////////////////////////////////////////////////////////

#[derive(Copy, Clone)]
pub struct Button<'a, MSG> {
    label: &'a str,
    on_click: MSG,
}

////////////////////////////////////////////////////////////////
// API //
////////////////////////////////////////////////////////////////

pub fn button<'a, MSG>(label: &'a str, on_click: MSG) -> Button<'a, MSG> {
    Button {
        label: label,
        on_click,
    }
}

impl<T> Button<'static, T>
where
    T: 'static,
    T: Clone,
{
    pub fn view(self) -> Node<T> {
        button![self.label, ev(Ev::Click, |_| self.on_click)]
    }
}
