use crate::game_state;

trait Way {
    fn play<G: game_state::GameState, const P: usize>(_g: &mut G);
}
