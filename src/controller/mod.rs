use crate::kingdom;
use crate::game;

mod big_money;

pub trait Controller<K: kingdom::Kingdom> {
    fn act(&mut self);
    fn buy(&mut self, game: &mut game::Game<K, 2>, hand: &mut game::PersonalState);
}
