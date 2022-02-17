use crate::kingdom;
use crate::game;

pub mod big_money;
pub mod big_money_naive;

pub trait Controller<G: game::GameState> {
    fn act(&mut self);
    fn buy<const P: usize>(&mut self, game: &mut G);
}
