use crate::game;

pub mod big_money;
pub mod patrol;
pub mod patrol_harem;
pub mod smithy;
pub mod smithy_accidental_village;

pub trait Controller {
    fn act<G: game::GameState, const P: usize>(&mut self, _game: &mut G) {}
    fn buy<G: game::GameState, const P: usize>(&mut self, _game: &mut G) {}
}
