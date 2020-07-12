use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////
// GameRequest //
////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize)]
pub struct GameRequest {
    game_name: String,
    player_name: String,
}

impl GameRequest {
    pub fn init(game_name: String, player_name: String) -> GameRequest {
        GameRequest {
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
    pub fn from_bytes(byte_data: Vec<u8>) -> bincode::Result<GameRequest> {
        bincode::deserialize(&byte_data[..])
    }
}

/////////////////////////////////////////////////////////////////////
/// Helper Function,
/// This function takes in the string messages from the server
/// and converts them to their valid struct types.

pub fn from_hex_data(byte_data: String) -> Vec<u8> {
    match hex::decode(byte_data) {
        Ok(data) => data,
        Err(_error) => Vec::new(),
    }
}

////////////////////////////////////////////////////////////////
// JoinRequest //
////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize)]
pub struct JoinRequest {
    player_name: String,
    game_id: String,
}

impl JoinRequest {
    pub fn init(player_name: String, game_id: String) -> JoinRequest {
        JoinRequest {
            player_name,
            game_id,
        }
    }
    pub fn player_name(&self) -> String {
        self.player_name.clone()
    }
    pub fn game_id(&self) -> String {
        self.game_id.clone()
    }
    pub fn to_bytes(&self) -> bincode::Result<Vec<u8>> {
        bincode::serialize(self)
    }
    pub fn from_bytes(byte_data: Vec<u8>) -> bincode::Result<JoinRequest> {
        bincode::deserialize(&byte_data[..])
    }
}

////////////////////////////////////////////////////////////////
// Response //
////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Clone)]
pub struct Response {
    game_id: String,
}

impl Response {
    pub fn init(game_id: String) -> Response {
        Response { game_id }
    }

    pub fn get_game_id(&self) -> String {
        self.game_id.clone()
    }

    pub fn to_bytes(&self) -> bincode::Result<Vec<u8>> {
        bincode::serialize(self)
    }

    pub fn from_bytes(byte_data: Vec<u8>) -> bincode::Result<Response> {
        bincode::deserialize(&byte_data[..])
    }
}

////////////////////////////////////////////////////////////////
// JoinResponse //
////////////////////////////////////////////////////////////////
#[derive(Serialize, Deserialize, Clone)]
pub struct JoinResponse {
    game_id: String,
    game_host: String,
    num_players: usize,
}

impl JoinResponse {
    pub fn init(game_id: String, game_host: String, num_players: usize) -> JoinResponse {
        JoinResponse {
            game_id,
            game_host,
            num_players,
        }
    }
    pub fn get_game_host(&self) -> String {
        self.game_host.clone()
    }
    pub fn get_num_players(&self) -> usize {
        self.num_players
    }

    pub fn get_game_id(&self) -> String {
        self.game_id.clone()
    }

    pub fn to_bytes(&self) -> bincode::Result<Vec<u8>> {
        bincode::serialize(self)
    }

    pub fn from_bytes(byte_data: Vec<u8>) -> bincode::Result<JoinResponse> {
        bincode::deserialize(&byte_data[..])
    }
}
