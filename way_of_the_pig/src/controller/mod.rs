use crate::game;

pub mod big_money;
pub mod big_money_naive;
pub mod smithy;

pub trait Controller{
    fn act(&mut self);
    fn buy<G: game::GameState, const P: usize>(&mut self, game: &mut G);
}
