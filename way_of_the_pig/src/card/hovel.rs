use crate::card;
use crate::game;

pub struct Card {}

impl card::Card for Card {
    #[inline]
    fn static_price() -> u32 {
        1
    }

    #[inline]
    fn static_type() -> card::Type {
        card::Type::REACTION | card::Type::SHELTER
    }
}
