use yew::prelude::{App, Component, ComponentLink, Html};

use ui::view::element;

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

enum Msg {
    AddOne,
}

////////////////////////////////////////////////////////////////
// UPDATE //
////////////////////////////////////////////////////////////////

fn update(msg: Msg, model: &mut Model) {
    match msg {
        Msg::AddOne => {
            model.value += 1;
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        update(msg, self);
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        return element::node(vec![
            element::text("TRIPLE"),
            element::text(" "),
            element::text("KILL"),
        ])
        .to_html(|msg: Msg| msg);
    }
}

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
}
