use crate::pile;
use crate::game;

pub struct Pile {
    card: u8,
}

impl pile::Pile for Pile {
    #[inline]
    fn make() -> Self {
        // FIXME depend on player count
        Pile {
            card: 8,
        }
    }

    #[inline]
    fn enabled(&self) -> bool {
        true
    }

    #[inline]
    fn top(&self) -> Option<game::CardType> {
        if self.card == 0 {
            return None;
        }
        Some(game::CardType::Colony)
    }

    #[inline]
    fn pop(&mut self) -> Option<game::CardType> {
        if self.card == 0 {
            return None;
        }
        self.card -= 1;
        Some(game::CardType::Colony)
    }

    #[inline]
    fn remaining_cards(&self) -> u8 {
        self.card
    }
}
