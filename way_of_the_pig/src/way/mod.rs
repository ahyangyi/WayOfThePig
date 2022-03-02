use crate::game;

trait Way {
    fn play<G: game::GameState, const P: usize>(_g: &mut G);
}
