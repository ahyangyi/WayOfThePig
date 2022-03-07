use crate::card;
use crate::game;

pub struct Card {}

impl card::Card for Card {
    #[inline]
    fn static_price() -> u32 {
        3
    }

    #[inline]
    fn static_type() -> card::Type {
        card::Type::ACTION
    }

    #[inline]
    fn play<G: game::GameState, const P: usize>(g: &mut G) {
        g.get_player::<P>().action += 2;
        g.draw::<P>();
    }
}
