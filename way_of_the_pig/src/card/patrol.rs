use crate::card;
use crate::card::CardType;
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
        for _card in 0..3 {
            g.draw::<P>();
        }

        let mut m: Vec<CardType> = vec![];
        for _i in 0..4 {
            let card = g.draw_to::<P>();
            match card {
                None => {
                    break;
                }
                Some(x) => {
                    if card::static_type(x).contains(card::Type::VICTORY) {
                        g.get_player::<P>().hand[x as usize] += 1;
                    } else {
                        m.push(x);
                    }
                }
            }
        }

        // FIXME: let the player to decide the order
        g.get_player::<P>().deck.append(&mut m);
    }
}
