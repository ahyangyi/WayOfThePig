use crate::card;
use crate::game;

pub struct Card {}

impl card::Card for Card {
    #[inline]
    fn static_price() -> u32 {
        5
    }

    #[inline]
    fn static_type() -> card::Type {
        card::Type::ACTION
    }

    #[inline]
    fn on_play<G: game::GameState, const P: usize>(g: &mut G) {
        g.get_player::<P>().action += 2;
        g.get_player::<P>().buy += 1;
        g.get_player::<P>().coin += 2;
    }
}
