use super::game::Game;
// use rand::{Rng, SeedableRng, StdRng};
use std::collections::HashMap;

////////////////////////////////////////////////////////////////
// Types //
////////////////////////////////////////////////////////////////

pub struct Model {
    /// String is an id
    games: HashMap<String, Game>,
    randomness_seed: u64,
}

////////////////////////////////////////////////////////////////
// Init //
////////////////////////////////////////////////////////////////

pub fn init(randomness_seed: u64) -> Model {
    // let mut rng: StdRng = SeedableRng::from_seed(randomness_seed);

    return Model {
        games: HashMap::new(),
        randomness_seed,
    };
}

////////////////////////////////////////////////////////////////
// Api //
////////////////////////////////////////////////////////////////

impl Model {
    pub fn games_count(&self) -> usize {
        self.games.len()
    }
}
