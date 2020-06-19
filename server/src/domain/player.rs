////////////////////////////////////////////////////////////////
// Types //
////////////////////////////////////////////////////////////////

pub struct Player {
    name: String,
    /// None means the player has not submitted moves for this turn
    /// Some(...) means they have, including the possibility that
    /// they submitted an empty list of moves.
    moves: Option<Vec<Move>>,
}

enum Move {
    Move,
}

pub fn init(name: String) -> Player {
    Player { name, moves: None }
}

impl Player {
    pub fn get_name(&self) -> String {
        self.name.to_string()
    }
    pub fn update_name(&mut self, new_name: String) {
        self.name = new_name
    }
}
