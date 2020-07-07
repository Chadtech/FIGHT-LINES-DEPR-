#![allow(clippy::wildcard_imports)]
use crate::route::Route;
use crate::session::{Session, WindowSize};
use crate::view::text::text;
use seed::{prelude::*, *};
use web_sys;

mod page;
mod route;
mod session;
mod util;
mod view;

////////////////////////////////////////////////////////////////
// TYPES //
////////////////////////////////////////////////////////////////

enum Program {
    Running(Model),
    Failed(String),
}

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
    WindowResized(Result<session::WindowSize, String>),
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

fn after_mount(url: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Program> {
    orders.send_msg(Msg::RouteChanged(route::parse(url)));

    orders.after_next_render(Msg::Rendered);

    orders.stream(streams::window_event(Ev::Resize, |_| {
        Msg::WindowResized(get_window_size())
    }));

    let program = match get_window_size() {
        Ok(window_size) => Program::Running(Model {
            page: Page::PageNotFound,
            session: session::init(window_size),
        }),
        Err(error) => Program::Failed(error),
    };

    AfterMount::new(program).url_handling(UrlHandling::PassToRoutes)
}

fn get_window_size() -> Result<session::WindowSize, String> {
    fn get_window_size_js_error() -> Result<session::WindowSize, JsValue> {
        match web_sys::window() {
            None => Err(JsValue::from_str("Window could not be found")),
            Some(window) => {
                let inner_width_js = window.inner_width()?;
                let inner_height_js = window.inner_height()?;

                let maybe_inner_width = inner_width_js.as_f64();
                let maybe_inner_height = inner_height_js.as_f64();

                match (maybe_inner_width, maybe_inner_height) {
                    (Some(inner_width), Some(inner_height)) => Ok(session::WindowSize {
                        width: inner_width as u16,
                        height: inner_height as u16,
                    }),
                    _ => Err(JsValue::from_str(
                        "Got window, but failed to ascertain its size",
                    )),
                }
            }
        }
    }
    get_window_size_js_error().map_err(|js_error| {
        js_error
            .as_string()
            .unwrap_or("Unknown error getting window size".to_string())
    })
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
        Msg::WindowResized(resize_result) => {
            match resize_result {
                Ok(new_window_size) => {
                    model.session.set_window_size(new_window_size);
                }
                Err(error) => {}
            }
            log!(model.session.get_window_size().width);
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
// PROGRAM //
////////////////////////////////////////////////////////////////

#[wasm_bindgen(start)]
pub fn start() {
    App::builder(update_program, view_program)
        .before_mount(before_mount)
        .after_mount(after_mount)
        .routes(|url| Some(Msg::RouteChanged(route::parse(url))))
        .build_and_start();
}

fn update_program(msg: Msg, program: &mut Program, orders: &mut impl Orders<Msg>) {
    if let Program::Running(model) = program {
        update(msg, model, orders);
    }
}

fn view_program(program: &Program) -> Node<Msg> {
    match program {
        Program::Running(model) => view(model),
        Program::Failed(error) => text(error),
    }
}
