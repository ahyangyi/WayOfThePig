use crate::pile;
use crate::game;

pub struct Pile {
    province: u8,
}

impl pile::Pile for Pile {
    #[inline]
    fn make() -> Self {
        // FIXME depend on player count
        Pile {
            province: 8,
        }
    }

    #[inline]
    fn enabled(&self) -> bool {
        true
    }

    #[inline]
    fn top(&self) -> Option<game::CardType> {
        if self.province == 0 {
            return None;
        }
        Some(game::CardType::Province)
    }

    #[inline]
    fn pop(&mut self) -> Option<game::CardType> {
        if self.province == 0 {
            return None;
        }
        self.province -= 1;
        Some(game::CardType::Province)
    }

    #[inline]
    fn remaining_cards(&self) -> u8 {
        self.province
    }
}
