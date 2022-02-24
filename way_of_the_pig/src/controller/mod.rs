use crate::game;

pub mod big_money;
pub mod big_money_naive;
pub mod smithy;
pub mod patrol;

pub trait Controller{
    fn act<G: game::GameState, const P: usize>(&mut self, _game: &mut G) {}
    fn buy<G: game::GameState, const P: usize>(&mut self, _game: &mut G) {}
}
