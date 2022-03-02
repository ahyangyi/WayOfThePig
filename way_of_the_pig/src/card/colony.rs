use crate::card;

pub struct Card {}

impl card::Card for Card {
    #[inline]
    fn static_price() -> u32 {
        11
    }
    #[inline]
    fn static_type() -> card::Type {
        card::Type::VICTORY
    }
}
