use crate::game;
use crate::card;

pub struct Card {
}

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
    fn play<G: game::GameState, const P: usize>(&self, g: &mut G) {
        for _card in 0..3 {
            g.get_player::<P>().draw();
        }
    }
}
