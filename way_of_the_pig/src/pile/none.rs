use crate::card;
use crate::pile;

pub struct Pile {}

impl pile::Pile for Pile {
    #[inline]
    fn make<const N: usize>() -> Self {
        Pile {}
    }

    #[inline]
    fn enabled(&self) -> bool {
        false
    }

    #[inline]
    fn top(&self) -> Option<card::CardType> {
        None
    }

    #[inline]
    fn pop(&mut self) -> Option<card::CardType> {
        None
    }

    #[inline]
    fn remaining_cards(&self) -> u8 {
        0
    }
}
