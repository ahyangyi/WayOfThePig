use crate::game;
use crate::card;

pub struct Card {
}

impl card::Card for Card {
    #[inline]
    fn static_price() -> u32 {
        6
    }

    #[inline]
    fn static_type() -> card::Type {
        card::Type::TREASURE
    }

    #[inline]
    fn play<G: game::GameState, const P: usize>(&self, g: &mut G) {
        g.get_player::<P>().coin += 3;
    }
}
