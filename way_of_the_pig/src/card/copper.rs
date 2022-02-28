use crate::game;
use crate::card;

pub struct Card {
}

impl card::Card for Card {
    fn static_price() -> u32 {
        0
    }
    fn static_type() -> card::Type {
        card::Type::TREASURE
    }
    fn play<G: game::GameState, const P: usize>(g: &mut G) {
        g.get_player::<P>().coin += 1;
    }
}
