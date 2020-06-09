use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GameRequest {
    gameName: String,
    numPlayers: i32,
}

impl GameRequest {
    pub fn init(name: String) -> GameRequest {
        GameRequest {
            gameName: name,
            numPlayers: 0,
        }
    }

    pub fn game_name(&self) -> String {
        self.gameName.to_string()
    }
    pub fn num_players(&self) -> i32 {
        self.numPlayers
    }
    pub fn set_game_name(&mut self, new_name: String) {
        self.gameName = new_name
    }
    pub fn set_num_players(&mut self, new_size: i32) {
        self.numPlayers = new_size
    }
    pub fn get_bytes_array(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
    pub fn obj_from_bytes(byte_data: Vec<u8>) -> GameRequest {
        let decoded: GameRequest = bincode::deserialize(&byte_data[..]).unwrap();
        decoded
    }
}
