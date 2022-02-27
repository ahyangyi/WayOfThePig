pub mod province;

pub trait Pile {
    fn make() -> Self;
    fn enabled(&self) -> bool;
    fn pop(&mut self);
    fn remaining_cards(&self) -> u8;
}
