use crate::card;
use crate::game;

pub struct Card {}

impl card::Card for Card {
    #[inline]
    fn static_price() -> u32 {
        4
    }

    #[inline]
    fn static_type() -> card::Type {
        card::Type::ACTION | card::Type::ATTACK
    }

    #[inline]
    fn on_play<G: game::Game, const P: usize>(g: &mut G) {
        g.get_player::<P>().coin += 1;
        // FIXME attack
    }
}
