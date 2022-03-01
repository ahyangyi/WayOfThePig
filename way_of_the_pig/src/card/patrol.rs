use crate::game;
use crate::card::CardType;
use crate::card;

pub struct Card {
}

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
    fn play<G: game::GameState, const P: usize>(g: &mut G) {
        for _card in 0..3 {
            g.get_player::<P>().draw();
        }

        let mut m : Vec<CardType> = vec![];
        for _i in 0..4 {
            let card = g.get_player::<P>().draw_to();
            match card {
                None => {break;},
                Some(x) => {
                    if x == CardType::Province || x == CardType::Duchy || x == CardType::Estate {
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
