use crate::card;
use crate::game;

pub struct Card {}

impl card::Card for Card {
    #[inline]
    fn static_price() -> u32 {
        9
    }

    #[inline]
    fn static_type() -> card::Type {
        card::Type::TREASURE
    }

    #[inline]
    fn play<G: game::GameState, const P: usize>(g: &mut G) {
        g.get_player::<P>().coin += 5;
    }
}
