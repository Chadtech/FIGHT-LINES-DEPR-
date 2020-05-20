use fightlines_moves::moves::Move;

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

pub fn init(name: &str) -> Player {
    Player {
        name: name.to_string(),
        moves: None,
    }
}

impl Player {
    pub fn get_name(&self) -> String {
        self.name.to_string()
    }
    pub fn update_name(&mut self, new_name: String) {
        self.name = new_name
    }
    

}
