#![allow(clippy::wildcard_imports)]
use crate::route::Route;
use crate::session::Session;
use seed::{prelude::*, *};

mod page;
mod route;
mod session;
mod util;
mod view;

////////////////////////////////////////////////////////////////
// TYPES //
////////////////////////////////////////////////////////////////

struct Model {
    session: Session,
    page: Page,
}

enum Page {
    PageNotFound,
    Title,
    StartGame(page::start_game::Model),
    Demo(page::demo::Form),
    Lobby(page::lobby::Model),
    Game(page::game::Model),
}

enum Msg {
    RouteChanged(Option<Route>),
    Rendered(RenderInfo),
    StartGame(page::start_game::Msg),
    Demo(page::demo::Msg),
    Lobby(page::lobby::Msg),
    Game(page::game::Msg),
}

////////////////////////////////////////////////////////////////
// PRIVATE HELPERS //
////////////////////////////////////////////////////////////////

impl Model {
    pub fn set_page(&mut self, page: Page) {
        self.page = page;
    }
}

////////////////////////////////////////////////////////////////
// INIT //
////////////////////////////////////////////////////////////////

fn before_mount(_: Url) -> BeforeMount {
    BeforeMount::new().mount_type(MountType::Takeover)
}

fn after_mount(url: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    orders.send_msg(Msg::RouteChanged(route::parse(url)));

    orders.after_next_render(Msg::Rendered);

    // TODO we need some kind of logic to determine
    // if we should use `init_dev()`, because in some
    // cases, like during a real deployment, we dont want a
    // dev session
    AfterMount::new(Model {
        page: Page::PageNotFound,
        session: session::init_dev(),
    })
    .url_handling(UrlHandling::PassToRoutes)
}

////////////////////////////////////////////////////////////////
// UPDATE //
////////////////////////////////////////////////////////////////

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::RouteChanged(maybe_route) => {
            handle_route(maybe_route, model);
        }
        Msg::Rendered(render_info) => {
            model
                .session
                .set_current_time(render_info.timestamp)
                .set_render_delta(render_info.timestamp_delta);
        }
        Msg::StartGame(sub_msg) => {
            if let Page::StartGame(sub_model) = &mut model.page {
                page::start_game::update(
                    sub_msg,
                    sub_model,
                    &model.session,
                    &mut orders.proxy(Msg::StartGame),
                )
            }
        }
        Msg::Demo(sub_msg) => {
            if let Page::Demo(sub_model) = &mut model.page {
                page::demo::update(sub_msg, sub_model)
            }
        }
        Msg::Lobby(sub_msg) => {
            if let Page::Lobby(sub_model) = &mut model.page {
                page::lobby::update(sub_msg, sub_model)
            }
        }
        Msg::Game(sub_msg) => {
            if let Page::Game(sub_model) = &mut model.page {
                page::game::update(sub_msg, sub_model)
            }
        }
    }

    orders.after_next_render(Msg::Rendered);
}

fn handle_route(maybe_route: Option<Route>, model: &mut Model) {
    match maybe_route {
        None => model.set_page(Page::PageNotFound),
        Some(route) => match route {
            Route::Title => match model.page {
                Page::Title => {}
                _ => model.set_page(Page::Title),
            },
            Route::StartGame => match model.page {
                Page::StartGame(_) => {}
                _ => model.set_page(Page::StartGame(page::start_game::init())),
            },
            Route::Demo => match model.page {
                Page::Demo(_) => {}
                _ => model.set_page(Page::Demo(page::demo::init())),
            },
            Route::Lobby(game_id) => match &model.page {
                Page::Lobby(sub_model) => {
                    if sub_model.get_game_id() != game_id {
                        model.set_page(Page::Lobby(page::lobby::init(game_id)));
                    }
                }
                _ => model.set_page(Page::Lobby(page::lobby::init(game_id))),
            },
            Route::Game(game_id) => match &model.page {
                Page::Game(sub_model) => {
                    if sub_model.get_game_id() != game_id {
                        model.set_page(Page::Game(page::game::init(game_id)));
                    }
                }
                _ => model.set_page(Page::Game(page::game::init(game_id))),
            },
        },
    }
}

////////////////////////////////////////////////////////////////
// VIEW //
////////////////////////////////////////////////////////////////

fn view(model: &Model) -> Node<Msg> {
    let body: Vec<Node<Msg>> = match &model.page {
        Page::Title => page::title::view(),
        Page::PageNotFound => vec![div!["Page not found!"]],
        Page::StartGame(sub_model) => page::start_game::view(&sub_model)
            .into_iter()
            .map(|node| node.map_msg(Msg::StartGame))
            .collect(),
        Page::Demo(sub_model) => page::demo::view(&sub_model)
            .into_iter()
            .map(|node| node.map_msg(Msg::Demo))
            .collect(),
        Page::Lobby(sub_model) => page::lobby::view(&sub_model)
            .into_iter()
            .map(|node| node.map_msg(Msg::Lobby))
            .collect(),
        Page::Game(sub_model) => page::game::view(&sub_model, &model.session)
            .into_iter()
            .map(|node| node.map_msg(Msg::Game))
            .collect(),
    };

    div![nodes![body]]
}

////////////////////////////////////////////////////////////////
// VIEW //
////////////////////////////////////////////////////////////////

#[wasm_bindgen(start)]
pub fn start() {
    App::builder(update, view)
        .before_mount(before_mount)
        .after_mount(after_mount)
        .routes(|url| Some(Msg::RouteChanged(route::parse(url))))
        .build_and_start();
}
