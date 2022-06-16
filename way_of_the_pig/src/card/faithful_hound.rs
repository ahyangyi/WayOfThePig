use crate::card;
use crate::game;

pub struct Card {}

impl card::Card for Card {
    #[inline]
    fn static_price() -> u32 {
        2
    }

    #[inline]
    fn static_type() -> card::Type {
        card::Type::ACTION | card::Type::REACTION
    }

    #[inline]
    fn on_play<G: game::Game, const P: usize>(g: &mut G) {
        for _card in 0..2 {
            g.draw::<P>();
        }
    }
}
