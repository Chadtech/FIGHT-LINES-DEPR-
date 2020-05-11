use yew::prelude::{App, Component, ComponentLink, Html};

use ui::page::title;
use yew::virtual_dom;
use yew::virtual_dom::vlist::VList;

////////////////////////////////////////////////////////////////
// Types //
////////////////////////////////////////////////////////////////

struct Program {
    // I think this might be necessary for connecting Msg
    // to the update cycle. But I dont know yet, and
    // clippy complains that it is unused
    // link: ComponentLink<Self>,
    model: Model,
}

enum Model {
    Title(title::Model),
}

enum Msg {
    TitleMsg(title::Msg),
}

////////////////////////////////////////////////////////////////
// Update //
////////////////////////////////////////////////////////////////

fn update(msg: Msg, _app: &mut Program) {
    match msg {
        Msg::TitleMsg(_sub_msg) => {}
    }
}

impl Component for Program {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            // link,
            model: Model::Title(title::init()),
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        update(msg, self);
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let page = match self.model {
            Model::Title(sub_model) => title::view(sub_model)
                .into_iter()
                .map(|child| child.map(Msg::TitleMsg).to_html(|msg: Msg| msg))
                .collect(),
        };

        virtual_dom::VNode::VList(VList::new_with_children(page, None))
        // return element::node(page).to_html(|msg: Msg| msg);
    }
}

fn main() {
    yew::initialize();
    App::<Program>::new().mount_to_body();
}
