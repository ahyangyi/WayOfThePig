use crate::card;

pub struct Card {}

impl card::Card for Card {
    fn static_price() -> u32 {
        8
    }
    fn static_type() -> card::Type {
        card::Type::VICTORY
    }
    fn vp() -> u32 {
        6
    }
}
