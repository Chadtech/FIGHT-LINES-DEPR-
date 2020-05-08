use super::game::Game;
use rand::{Rng, SeedableRng, StdRng};
use serde::Deserialize;
use std::collections::HashMap;
////////////////////////////////////////////////////////////////
// Types //
////////////////////////////////////////////////////////////////

pub struct Model {
    /// String is an id
    games: HashMap<u64, Game>,
    randomness_seed: usize,
}

////////////////////////////////////////////////////////////////
// Init //
////////////////////////////////////////////////////////////////

pub fn init(randomness_seed: usize) -> Model {
    Model {
        games: HashMap::new(),
        randomness_seed,
    }
}

////////////////////////////////////////////////////////////////
// Api //
////////////////////////////////////////////////////////////////

impl Model {
    pub fn games_count(&self) -> usize {
        self.games.len()
    }
    pub fn random_seed(&self) -> usize {
        self.randomness_seed
    }
    pub fn add_game(&mut self, new_game: Game) {
        let mut rng = self.get_rng();

        let id: u64 = rng.gen();

        self.games.insert(id, new_game);
    }

    fn set_seed(&mut self, randomness_seed: usize) {
        self.randomness_seed = randomness_seed;
    }

    /// If we ever need a random value, we need a
    /// random number generator. To get the random
    /// number generator, we should always use get_rng
    /// because that will automatically update the
    /// randomness_seed inside the model. If we dont
    /// update it we will re-use the same seed, which
    /// would lead to weird things like every random
    /// event coming out the same way (every attack
    /// always doing the same damage, for example)
    fn get_rng(&mut self) -> StdRng {
        let seed: &[usize] = &[self.randomness_seed];

        let mut rng: StdRng = SeedableRng::from_seed(seed);

        let new_randomness_seed: usize = rng.gen();

        self.set_seed(new_randomness_seed);

        rng
    }
}

#[derive(Deserialize)]
pub struct FormData {
    pub game_name: String,
    pub num_players: i64,
}

////////////////////////////////////////////////////////////////
// Tests //
////////////////////////////////////////////////////////////////

#[cfg(test)]
mod model_tests {
    use crate::domain::model::Model;
    use crate::domain::{game, model};

    fn init_test_model() -> Model {
        model::init(0)
    }
    #[test]
    fn add_game_increases_game_count() {
        let mut test_model = init_test_model();

        test_model.add_game(game::init("test game"));

        assert_eq!(test_model.games_count(), 1);
    }

    #[test]
    fn using_the_seed_changes_it() {
        let mut test_model_0 = init_test_model();
        let mut test_model_1 = init_test_model();

        let _rng = test_model_0.get_rng();

        assert_ne!(test_model_0.randomness_seed, test_model_1.randomness_seed)
    }
}
