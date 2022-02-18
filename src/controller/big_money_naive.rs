// A weaker Big Money bot than "Big Money Ultimate". Strategy:
//   Buy province if total money > 18
//   Buy gold
//   Buy silver
use crate::game;
use crate::controller;
use std::marker::PhantomData;

pub struct BigMoneyController<G: game::GameState> {
    game: PhantomData<G>,
}

fn total_money<G: game::GameState, const P: usize>(game: &mut G) -> u32 {
    game.get_player::<P>().count_card(game::CardType::Gold) * 3 +
    game.get_player::<P>().count_card(game::CardType::Silver) * 2 +
    game.get_player::<P>().count_card(game::CardType::Copper) * 1
}

impl<G: game::GameState> BigMoneyController<G> {
    pub fn make() -> BigMoneyController<G> {
        BigMoneyController {
            game: PhantomData,
        }
    }
}

impl<G: game::GameState> controller::Controller<G> for BigMoneyController<G> {
    fn act(&mut self) {
    }
    fn buy<const P: usize>(&mut self, game: &mut G) {
        while game.get_player::<P>().play_gold() {}
        while game.get_player::<P>().play_silver() {}
        while game.get_player::<P>().play_copper() {}
        if total_money::<G, P>(game) > 18 && game.buy_province::<P>() {
            return;
        } else if game.buy_gold::<P>() {
            return;
        } else {
            game.buy_silver::<P>();
        }
    }
}
