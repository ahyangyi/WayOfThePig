use crate::pile;
use crate::game;

pub struct Pile {
}

impl pile::Pile for Pile {
    #[inline]
    fn make() -> Self {
        Pile{}
    }

    #[inline]
    fn enabled(&self) -> bool {
        false
    }

    #[inline]
    fn top(&self) -> Option<game::CardType> {
        return None;
    }

    #[inline]
    fn pop(&mut self) -> Option<game::CardType> {
        return None;
    }

    #[inline]
    fn remaining_cards(&self) -> u8 {
        0
    }
}
