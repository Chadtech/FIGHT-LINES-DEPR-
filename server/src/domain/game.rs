use super::player;
use super::player::Player;
////////////////////////////////////////////////////////////////
// Types //
////////////////////////////////////////////////////////////////

pub enum Game {
    Lobby(LobbyModel),
}

pub struct LobbyModel {
    metadata: Metadata,
    host: Player,
    opponents: Vec<Player>,
}

pub struct Metadata {
    name: String,
}

////////////////////////////////////////////////////////////////
// Init //
////////////////////////////////////////////////////////////////

/// The parameters for this should be
/// all the things needed to make
/// a new game, but nothing else
pub fn init_lobby(name: String, host_name: String) -> Game {
    Game::Lobby(LobbyModel {
        metadata: Metadata { name: name.clone() },
        host: player::init(host_name.clone()),
        opponents: Vec::new(),
    })
}

/// Api
impl Game {
    pub fn game_name(self) -> String {
        match self {
            Game::Lobby(lobby) => lobby.metadata.name,
        }
    }

    pub fn num_players(&self) -> usize {
        match self {
            Game::Lobby(lobby) => lobby.opponents.len() + 1,
        }
    }

    pub fn add_player(&mut self, new_player: String) {
        match self {
            Game::Lobby(lobby) => {
                lobby.opponents.push(player::init(new_player));
            }
        }
    }
}

/// ////////////////////////
/// Tests
/// ///////////////////////

#[cfg(test)]
mod game_test {
    use crate::domain::game;
    use crate::domain::game::Game;

    fn init_game_obj() -> Game {
        game::init_lobby("Hello World".to_string(), "Hank".to_string())
    }

    #[test]
    fn update_players() {
        let mut game_model = init_game_obj();
        game_model.add_player("Harribel".to_string());
        game_model.add_player("Lisa".to_string());
        game_model.add_player("Chen".to_string());

        assert_eq!(game_model.num_players(), 4)
    }
}
