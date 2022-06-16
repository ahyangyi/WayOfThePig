use crate::card;
use crate::game_state;

pub struct Card {}

impl card::Card for Card {
    #[inline]
    fn static_price() -> u32 {
        4
    }

    #[inline]
    fn static_type() -> card::Type {
        card::Type::ACTION
    }

    #[inline]
    fn on_play<G: game_state::GameState, const P: usize>(g: &mut G) {
        for _card in 0..3 {
            g.draw::<P>();
        }
    }
}
