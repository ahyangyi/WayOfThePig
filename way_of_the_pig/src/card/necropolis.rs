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
        card::Type::ACTION | card::Type::SHELTER
    }

    #[inline]
    fn on_play<G: game::GameState, const P: usize>(g: &mut G) {
        g.get_player::<P>().action += 2;
    }
}
