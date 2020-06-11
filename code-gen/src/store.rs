use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GameRequest {
    game_name: String,
    num_players: i32,
}

impl GameRequest {
    pub fn init(name: String) -> GameRequest {
        GameRequest {
            game_name: name,
            num_players: 0,
        }
    }

    pub fn game_name(&self) -> String {
        self.game_name.to_string()
    }
    pub fn num_players(&self) -> i32 {
        self.num_players
    }
    pub fn set_game_name(&mut self, new_name: String) {
        self.game_name = new_name
    }
    pub fn set_num_players(&mut self, new_size: i32) {
        self.num_players = new_size
    }
    pub fn get_bytes_array(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
    pub fn from_bytes(byte_data: Vec<u8>) -> GameRequest {
        let decoded: GameRequest = bincode::deserialize(&byte_data[..]).unwrap();
        decoded
    }
}
