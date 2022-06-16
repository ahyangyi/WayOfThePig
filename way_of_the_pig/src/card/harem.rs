use crate::card;
use crate::game_state;

pub struct Card {}

impl card::Card for Card {
    #[inline]
    fn static_price() -> u32 {
        6
    }

    #[inline]
    fn static_type() -> card::Type {
        card::Type::TREASURE | card::Type::VICTORY
    }

    #[inline]
    fn on_play<G: game_state::GameState, const P: usize>(g: &mut G) {
        g.add_coin::<P>(2);
    }
}
