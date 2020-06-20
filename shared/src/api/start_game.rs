use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////
// Request //
////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize)]
pub struct Request {
    game_name: String,
    player_name: String,
}

impl Request {
    pub fn init(game_name: String, player_name: String) -> Request {
        Request {
            game_name,
            player_name,
        }
    }

    pub fn game_name(&self) -> String {
        self.game_name.clone()
    }
    pub fn host_name(&self) -> String {
        self.player_name.clone()
    }
    pub fn to_bytes(&self) -> bincode::Result<Vec<u8>> {
        bincode::serialize(self)
    }
    pub fn from_bytes(byte_data: Vec<u8>) -> bincode::Result<Request> {
        bincode::deserialize(&byte_data[..])
    }
}

////////////////////////////////////////////////////////////////
// Response //
////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Clone)]
pub struct Response {
    game_id: u64,
}

impl Response {
    pub fn init(game_id: u64) -> Response {
        Response { game_id }
    }

    pub fn get_game_id(&self) -> u64 {
        self.game_id
    }

    pub fn to_bytes(&self) -> bincode::Result<Vec<u8>> {
        bincode::serialize(self)
    }

    pub fn from_bytes(byte_data: Vec<u8>) -> bincode::Result<Response> {
        bincode::deserialize(&byte_data[..])
    }
}
