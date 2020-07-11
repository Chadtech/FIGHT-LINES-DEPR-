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
    session: Session,
    join_game_id: String,
    request_state: RequestState,
}

enum RequestState {
    Ready,
    Waiting,
    Failed(String),
    Finished(String),
}

#[derive(Clone)]
pub enum Msg {
    StartGameClicked,
    GameNameFieldUpdated(String),
    PlayerNameFieldUpdated(String),
    GameIdFieldUpdated(String),
    NewGameResponse(start_game::Response),
    NewGameFailed(String),
    JoinGameClicked,
    JoinGameResponse(start_game::JoinResponse),
}

impl Model {
    pub fn get_session_mut(&mut self) -> &mut Session {
        &mut self.session
    }
    pub fn update_game_name(&mut self, new_name: String) {
        self.game_name_field = new_name;
    }

    pub fn update_player_name(&mut self, new_name: String) {
        self.player_name_field = new_name;
    }
    pub fn update_game_id(&mut self, new_id: String) {
        self.join_game_id = new_id;
    }

    pub fn record_error(&mut self, error_msg: String) {
        self.request_state = RequestState::Failed(error_msg);
    }
    pub fn waiting(&mut self) {
        log("Kanot");
        self.request_state = RequestState::Waiting;
    }

    pub fn ready_state(&mut self, game_id: String) {
        self.request_state = RequestState::Finished(game_id);
    }
}

////////////////////////////////////////////////////////////////
// INIT //
////////////////////////////////////////////////////////////////

pub fn init(session: Session) -> Model {
    Model {
        game_name_field: String::new(),
        player_name_field: String::new(),
        session,
        join_game_id: String::new(),
        request_state: RequestState::Ready,
    }
}

////////////////////////////////////////////////////////////////
// UPDATE //
////////////////////////////////////////////////////////////////

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::StartGameClicked => {
            let game_request = start_game::GameRequest::init(
                model.game_name_field.clone(),
                model.player_name_field.clone(),
            );

            let url = model.get_session_mut().url("/game/create");

            match game_request.to_bytes() {
                Ok(bytes) => {
                    model.waiting();
                    orders.skip().perform_cmd({
                        async {
                            match send_message(url, bytes).await {
                                Ok(bytes) => {
                                    let u8_array = start_game::Response::from_hex_data(bytes);
                                    match start_game::Response::from_bytes(u8_array) {
                                        Ok(response) => {
                                            log(response.get_game_id());
                                            Msg::NewGameResponse(response)
                                        }
                                        Err(error) => Msg::NewGameFailed(error.to_string()),
                                    }
                                }
                                Err(error) => {
                                    let fetch_error = util::http::fetch_error_to_string(error);
                                    Msg::NewGameFailed(fetch_error)
                                }
                            }
                        }
                    });
                }
                Err(error) => {
                    model.record_error(error.to_string());
                }
            }
        }
        Msg::GameNameFieldUpdated(new_field) => model.update_game_name(new_field),
        Msg::PlayerNameFieldUpdated(new_field) => model.update_player_name(new_field),
        Msg::GameIdFieldUpdated(new_field) => model.update_game_id(new_field),
        Msg::NewGameResponse(response) => {
            let game_id = response.get_game_id();
            log("NewGameResponse");
            model.ready_state(game_id)
            // TODO after we get the game_id we should navigate to the lobby page
        }

        Msg::NewGameFailed(error) => model.record_error(error),
        Msg::JoinGameResponse(response) => {
            let game_id = response.get_game_id();
        }
        Msg::JoinGameClicked => {
            let net_game_id: String = model.join_game_id.clone();
            log!(net_game_id);
            let game_request =
                start_game::JoinRequest::init(model.player_name_field.clone(), net_game_id);

            let url = model.get_session_mut().url("/game/join");
            //   match game_request.to_bytes() {
            //       Ok(bytes) => {
            //           model.waiting();
            //           orders.skip().perform_cmd({
            //               async {
            //                   let msg = match send_message(url, bytes).await {
            //                       Ok(bytes) => match start_game::Response::from_bytes(bytes) {
            //                           Ok(response) => Msg::NewGameResponse(response),
            //                           Err(error) => Msg::NewGameFailed(error.to_string()),
            //                       },
            //                       Err(error) => {
            //                           let fetch_error = util::http::fetch_error_to_string(error);
            //                           Msg::NewGameFailed(fetch_error)
            //                       }
            //                   };

            //                   msg
            //               }
            //           });
            //       }
            //       Err(error) => {
            //           model.record_error(error.to_string());
            //       }
            //   }
        }
    }
}

async fn send_message(url: String, bytes: Vec<u8>) -> fetch::Result<String> {
    Request::new(url.as_str())
        .method(Method::Post)
        .text(hex::encode(bytes))
        .fetch()
        .await?
        .check_status()?
        .text()
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
            row::single(button("start", |_| Msg::StartGameClicked).view()),
            // Join Game Functionality
            row::single(text("______________")),
            row::single(text("Join Game")),
            row::single(
                text_field(model.player_name_field.as_str(), |event| {
                    Msg::PlayerNameFieldUpdated(event)
                })
                .placeholder("player name")
                .view(),
            ),
            row::single(
                text_field(&model.join_game_id, |event| Msg::GameIdFieldUpdated(event))
                    .placeholder("Game ID")
                    .view(),
            ),
            row::single(button("Join", |_| Msg::JoinGameClicked).view()),
        ])
        .map_rows(|row| row.center(true))
        .view(),
        RequestState::Waiting => vec![row::single(text("Waiting..")).view()],
        RequestState::Failed(error_msg) => vec![row::single(text(error_msg.as_str())).view()],
        RequestState::Finished(value) => {
            let mut str_message = String::new();
            str_message.push_str("Game_id: ");
            str_message.push_str(value);
            vec![row::single(text(&str_message.to_string())).view()]
        }
    }
}
