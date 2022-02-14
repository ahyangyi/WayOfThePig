use crate::kingdom;
use crate::game;

pub mod big_money;

pub trait Controller<K: kingdom::Kingdom, const N: usize> {
    fn act(&mut self);
    fn buy<const P: usize>(&mut self, game: &mut game::Game<K, N>);
}
