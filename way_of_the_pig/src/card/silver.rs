use crate::card;

pub struct Card {
}

impl card::Card for Card {
    fn static_price() -> u32 {
        3
    }
    fn static_type() -> card::Type {
        card::Type::TREASURE
    }
}
