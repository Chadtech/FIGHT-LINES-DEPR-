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
    return Game {
        name: name.to_string(),
        players: Vec::new(),
        round: 0,
    };
}
