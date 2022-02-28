// An implementation of DominionSim strategy "Smithy" by HiveMindEmulator, which attempts to buy the first
// possible entry:
//   Buy province if total money > 15
//   Buy duchy if remaining province <= 4
//   Buy estate if remaining province <= 2
//   Buy gold
//   Buy Duchy if remaining province <= 6
//   Buy Smithy if #smithy < #treasure / 11
//   Buy silver
use crate::game;
use crate::controller;

pub struct BigMoneyController {
}

fn total_money<G: game::GameState, const P: usize>(game: &mut G) -> u32 {
    game.get_player::<P>().count_card(game::CardType::Gold) * 3 +
    game.get_player::<P>().count_card(game::CardType::Silver) * 2 +
    game.get_player::<P>().count_card(game::CardType::Copper) * 1
}

fn num_money<G: game::GameState, const P: usize>(game: &mut G) -> u32 {
    game.get_player::<P>().count_card(game::CardType::Gold) +
    game.get_player::<P>().count_card(game::CardType::Silver) +
    game.get_player::<P>().count_card(game::CardType::Copper)
}

impl BigMoneyController {
    pub fn make() -> BigMoneyController{
        BigMoneyController {}
    }
}

impl controller::Controller for BigMoneyController {
    fn act<G: game::GameState, const P: usize>(&mut self, game: &mut G) {
        game.play_smithy::<P>();
    }
    fn buy<G: game::GameState, const P: usize>(&mut self, game: &mut G) {
        while game.play_gold::<P>() {}
        while game.play_silver::<P>() {}
        while game.play_copper::<P>() {}
        if total_money::<G, P>(game) > 15 && game.buy_province::<P>() {
            return;
        } else if game.province_in_supply() <= 4 && game.buy_duchy::<P>() {
            return;
        } else if game.province_in_supply() <= 2 && game.buy_estate::<P>() {
            return;
        } else if game.buy_gold::<P>() {
            return;
        } else if game.province_in_supply() <= 6 && game.buy_duchy::<P>() {
            return;
        } else if game.get_player::<P>().count_card(game::CardType::Smithy) * 11 < num_money::<G, P>(game) && game.buy_smithy::<P>() {
            return;
        } else {
            game.buy_silver::<P>();
        }
    }
}
