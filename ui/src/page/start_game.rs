use crate::session::Session;
use crate::view::button::button;
use crate::view::grid::row;
use crate::view::text::text;
use crate::view::text_field::text_field;
use seed::log;
use seed::prelude::{fetch, Method, Node, Orders, Request};
use serde::{Deserialize, Serialize};
use shared::game_request::GameRequest;

////////////////////////////////////////////////////////////////
// TYPES //
////////////////////////////////////////////////////////////////

pub struct Model {
    game_name_field: String,
    player_name_field: String,
    session: Session,
}

#[derive(Clone)]
pub enum Msg {
    StartClicked,
    GameNameFieldUpdated(String),
    PlayerNameFieldUpdated(String),
    NewGameResponse(Response),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Response {
    Yep,
    Nope,
}

impl Model {
    pub fn get_session(&self) -> Session {
        self.session
    }
    pub fn update_game_name(&mut self, new_name: String) {
        self.game_name_field = new_name;
    }

    pub fn update_player_name(&mut self, new_name: String) {
        self.player_name_field = new_name;
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
    }
}

////////////////////////////////////////////////////////////////
// UPDATE //
////////////////////////////////////////////////////////////////

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::StartClicked => {
            let game_request = GameRequest::init(model.game_name_field.clone());

            let url = model.get_session().url("/game/create");

            let byte_data = game_request.get_bytes_array();
            orders.skip().perform_cmd({
                async {
                    let response = match send_message(url, byte_data).await {
                        Ok(_) => Response::Yep,
                        Err(err) => {
                            log!(err);
                            Response::Nope
                        }
                    };
                    Msg::NewGameResponse(response)
                }
            });
        }
        Msg::GameNameFieldUpdated(new_field) => {
            model.update_game_name(new_field);
        }
        Msg::PlayerNameFieldUpdated(new_field) => {
            model.update_player_name(new_field);
        }
        Msg::NewGameResponse(response) => {
            log!(response);
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
    row::many(vec![
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
    .view()
}
