use ui::page::title;
use ui::program;
use ui::program::{Model, Msg, Program};
use yew::prelude::{App, Component, ComponentLink, Html};
use yew::virtual_dom;
use yew::virtual_dom::vlist::VList;

fn main() {
    yew::initialize();
    App::<Program>::new().mount_to_body();
}
