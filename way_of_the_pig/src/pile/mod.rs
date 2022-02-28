use crate::game;

pub mod none;

pub mod province;
pub mod colony;

pub trait Pile {
    fn make() -> Self;
    fn enabled(&self) -> bool;
    fn top(&self) -> Option<game::CardType>;
    fn pop(&mut self) -> Option<game::CardType>;
    fn remaining_cards(&self) -> u8;
}
