use crate::card;

pub struct Card {}

impl card::Card for Card {
    fn static_price() -> u32 {
        11
    }
    fn static_type() -> card::Type {
        card::Type::VICTORY
    }
}
