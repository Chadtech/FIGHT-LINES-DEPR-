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

////////////////////////////////////////////////////////////////
// JoinRequest //
////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize)]
pub struct JoinRequest {
    player_name: String,
    game_id: u64,
}

impl JoinRequest {
    pub fn init(game_id: u64, player_name: String) -> JoinRequest {
        JoinRequest {
            player_name,
            game_id,
        }
    }
    pub fn player_name(&self) -> String {
        self.player_name.clone()
    }
    pub fn game_id(&self) -> u64 {
        self.game_id
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

////////////////////////////////////////////////////////////////
// JoinResponse //
////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Clone)]
pub struct JoinResponse {
    game_id: u64,
    game_host: String,
    num_players: i64,
}

impl JoinResponse {
    pub fn init(game_id: u64, game_host: String, num_players: i64) -> JoinResponse {
        JoinResponse {
            game_id,
            game_host,
            num_players,
        }
    }
    pub fn get_game_host(&self) -> String {
        self.game_host.clone()
    }
    pub fn get_num_players(&self) -> i64 {
        self.num_players
    }

    pub fn get_game_id(&self) -> u64 {
        self.game_id
    }

    pub fn to_bytes(&self) -> bincode::Result<Vec<u8>> {
        bincode::serialize(self)
    }

    pub fn from_bytes(byte_data: Vec<u8>) -> bincode::Result<JoinResponse> {
        bincode::deserialize(&byte_data[..])
    }
}
