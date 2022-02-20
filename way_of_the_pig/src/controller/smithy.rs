// An implementation of DominionSim strategy "Smithy" by HiveMindEmulator, which attempts to buy the first
// possible entry:
//   Buy province if total money > 15
//   Buy duchy if remaining province <= 4
//   Buy estate if remaining province <= 2
//   Buy gold
//   Buy Duchy if remaining province <= 6
//   Buy Smithy if #smithy < #treasure / 11
//   Buy silver
use crate::kingdom;
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
        } else if game.province_in_supply() <= 4 && game.buy_duchy::<P>() {
            return;
        } else if game.province_in_supply() <= 2 && game.buy_estate::<P>() {
            return;
        } else if game.buy_gold::<P>() {
            return;
        } else if game.province_in_supply() <= 6 && game.buy_duchy::<P>() {
            return;
        } else {
            game.buy_silver::<P>();
        }
    }
}
