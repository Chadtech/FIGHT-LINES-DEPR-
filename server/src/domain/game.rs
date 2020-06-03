use super::player;
use super::player::Player;
////////////////////////////////////////////////////////////////
// Types //
////////////////////////////////////////////////////////////////

pub struct Game {
    name: String,
    players: Vec<Player>,
    round: i64,
}

////////////////////////////////////////////////////////////////
// Init //
////////////////////////////////////////////////////////////////

/// The parameters for this should be
/// all the things needed to make
/// a new game, but nothing else
pub fn init(name: &str) -> Game {
    Game {
        name: name.to_string(),
        players: Vec::new(),
        round: 0,
    }
}

/// Api
impl Game {
    pub fn game_name(&self) -> String {
        self.name.to_string()
    }

    pub fn game_round(&self) -> i64 {
        self.round
    }

    pub fn num_players(&self) -> usize {
        self.players.len()
    }

    pub fn update_game_name(&mut self, new_name: String) {
        self.name = new_name
    }

    pub fn add_player(&mut self, new_player: &str) {
        self.players.push(player::init(new_player));
    }
    pub fn update_round(&mut self) {
        self.round += 1
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
        game::init("Hello World")
    }
    #[test]
    fn update_name() {
        let mut game_model = init_game_obj();

        game_model.update_game_name(String::from("Antidote"));

        assert_eq!(game_model.name, "Antidote")
    }

    #[test]
    fn update_players() {
        let mut game_model = init_game_obj();
        game_model.add_player(&"Harribel");
        game_model.add_player(&"Lisa");
        game_model.add_player(&"Chen");

        assert_eq!(game_model.num_players(), 3)
    }
    #[test]
    fn update_rounds() {
        let mut game_model = init_game_obj();
        game_model.update_round();

        assert_eq!(game_model.round, 1);

        game_model.update_round();
        assert_eq!(game_model.round, 2);
        game_model.update_round();
        assert_eq!(game_model.round, 3);
    }
}
