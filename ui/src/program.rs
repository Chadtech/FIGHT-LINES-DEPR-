use crate::page::title;
use crate::view::element;
use yew::prelude::{App, Component, ComponentLink, Html};
use yew::services::console::ConsoleService;
use yew::virtual_dom;
use yew::virtual_dom::vlist::VList;

////////////////////////////////////////////////////////////////
// Types //
////////////////////////////////////////////////////////////////

pub struct Program {
    // I think this might be necessary for connecting Msg
    // to the update cycle. But I dont know yet, and
    // clippy complains that it is unused
    link: ComponentLink<Self>,
    pub model: Model,
}

#[derive(Clone, Copy)]
pub enum Model {
    Title(title::Model),
}

#[derive(Clone, Copy)]
pub enum Msg {
    TitleMsg(title::Msg),
}

////////////////////////////////////////////////////////////////
// Update //
////////////////////////////////////////////////////////////////

pub fn update(msg: Msg, _app: &mut Program) {
    match msg {
        Msg::TitleMsg(_sub_msg) => {
            let mut console = ConsoleService::new();
            console.log("C!!!")
        }
    }
}

impl Component for Program {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
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
        let link = &self.link;
        let page = match self.model {
            Model::Title(sub_model) => title::view(sub_model)
                .into_iter()
                .map(|child| child.html(link, Msg::TitleMsg))
                .collect(),
        };

        virtual_dom::VNode::VList(VList::new_with_children(page, None))
    }
}
