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
