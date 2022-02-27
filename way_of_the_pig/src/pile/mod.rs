use crate::game;

pub mod province;

pub trait Pile {
    fn make() -> Self;
    fn enabled(&self) -> bool;
    fn top(&self) -> Option<game::CardType>;
    fn pop(&mut self) -> Option<game::CardType>;
    fn remaining_cards(&self) -> u8;
}
