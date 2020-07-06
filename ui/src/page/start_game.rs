use crate::session::Session;
use crate::util;
use crate::view::button::button;
use crate::view::grid::row;
use crate::view::text::text;
use crate::view::text_field::text_field;
use seed::log;
use seed::prelude::{fetch, Method, Node, Orders, Request};
use shared::api::start_game;

////////////////////////////////////////////////////////////////
// TYPES //
////////////////////////////////////////////////////////////////

pub struct Model {
    game_name_field: String,
    player_name_field: String,
    request_state: RequestState,
}

enum RequestState {
    Ready,
    Waiting,
    Failed(String),
}

#[derive(Clone)]
pub enum Msg {
    StartClicked,
    GameNameFieldUpdated(String),
    PlayerNameFieldUpdated(String),
    NewGameResponse(start_game::Response),
    NewGameFailed(String),
}

impl Model {
    pub fn update_game_name(&mut self, new_name: String) {
        self.game_name_field = new_name;
    }

    pub fn update_player_name(&mut self, new_name: String) {
        self.player_name_field = new_name;
    }

    pub fn record_error(&mut self, error_msg: String) {
        self.request_state = RequestState::Failed(error_msg);
    }

    pub fn waiting(&mut self) {
        self.request_state = RequestState::Waiting;
    }
}

////////////////////////////////////////////////////////////////
// INIT //
////////////////////////////////////////////////////////////////

pub fn init() -> Model {
    Model {
        game_name_field: String::new(),
        player_name_field: String::new(),
        request_state: RequestState::Ready,
    }
}

////////////////////////////////////////////////////////////////
// UPDATE //
////////////////////////////////////////////////////////////////

pub fn update(msg: Msg, model: &mut Model, session: &Session, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::StartClicked => {
            let game_request = start_game::Request::init(
                model.game_name_field.clone(),
                model.player_name_field.clone(),
            );

            let url = session.url("/game/create");

            match game_request.to_bytes() {
                Ok(bytes) => {
                    log!("BYTES AS WE SEND THEM");
                    log!(bytes);
                    model.waiting();
                    orders.skip().perform_cmd({
                        async {
                            let msg = match send_message(url, bytes).await {
                                Ok(bytes) => match start_game::Response::from_bytes(bytes) {
                                    Ok(response) => Msg::NewGameResponse(response),
                                    Err(error) => Msg::NewGameFailed(error.to_string()),
                                },
                                Err(error) => {
                                    let fetch_error = util::http::fetch_error_to_string(error);
                                    Msg::NewGameFailed(fetch_error)
                                }
                            };

                            msg
                        }
                    });
                }
                Err(error) => {
                    model.record_error(error.to_string());
                }
            }
        }
        Msg::GameNameFieldUpdated(new_field) => {
            model.update_game_name(new_field);
        }
        Msg::PlayerNameFieldUpdated(new_field) => {
            model.update_player_name(new_field);
        }
        Msg::NewGameResponse(response) => {
            let game_id = response.get_game_id();
            model.record_error(game_id.to_string());

            // TODO after we get the game_id we should navigate to the lobby page
        }
        Msg::NewGameFailed(error) => {
            model.record_error(error);
        }
    }
}

async fn send_message(url: String, bytes: Vec<u8>) -> fetch::Result<Vec<u8>> {
    Request::new(url.as_str())
        .method(Method::Post)
        .text(hex::encode(bytes))
        .fetch()
        .await?
        .check_status()?
        .bytes()
        .await
}

////////////////////////////////////////////////////////////////
// VIEW //
////////////////////////////////////////////////////////////////

pub fn view(model: &Model) -> Vec<Node<Msg>> {
    match &model.request_state {
        RequestState::Ready => row::many(vec![
            row::single(text("start game")),
            row::single(
                text_field(model.player_name_field.as_str(), |event| {
                    Msg::PlayerNameFieldUpdated(event)
                })
                .placeholder("player name")
                .view(),
            ),
            row::single(
                text_field(model.game_name_field.as_str(), |event| {
                    Msg::GameNameFieldUpdated(event)
                })
                .placeholder("game name")
                .view(),
            ),
            row::single(button("start", |_| Msg::StartClicked).view()),
        ])
        .map_rows(|row| row.center(true))
        .view(),
        RequestState::Waiting => vec![row::single(text("Waiting..")).view()],
        RequestState::Failed(error_msg) => vec![row::single(text(error_msg.as_str())).view()],
    }
}
