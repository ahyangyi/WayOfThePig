// An implementation of WanderingWinder's "Big Money Ultimate", which attempts to buy the first possible entry:
//   Buy province if total money > 18
//   Buy duchy if remaining province <= 4
//   Buy estate if remaining province <= 2
//   Buy gold
//   Buy duchy if remaining province <= 6
//   Buy silver
// This implementation also supports the Colony variation, which is the following:
//   Buy colony if total money > 32
//   Buy province if remaining colony <= 6
//   Buy duchy if remaining colony <= 5
//   Buy estate if remaining colony <= 2
//   Buy platinum
//   Buy province if remaining colony <= 7
//   Buy gold
//   Buy duchy if remaining colony <= 6
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

impl BigMoneyController {
    pub fn make() -> BigMoneyController{
        BigMoneyController {}
    }
}

impl controller::Controller for BigMoneyController {
    fn act(&mut self) {
    }
    fn buy<G: game::GameState, const P: usize>(&mut self, game: &mut G) {
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
