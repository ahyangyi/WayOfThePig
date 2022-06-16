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
use crate::card;
use crate::controller;
use crate::game;

pub struct Controller {}

fn total_money<G: game::Game, const P: usize>(game: &mut G) -> u32 {
    game.get_player::<P>().count_card(card::CardType::Gold) * 3
        + game.get_player::<P>().count_card(card::CardType::Silver) * 2
        + game.get_player::<P>().count_card(card::CardType::Copper)
}

impl Controller {
    pub fn make() -> Controller {
        Controller {}
    }
}

impl controller::Controller for Controller {
    fn act<G: game::Game, const P: usize>(&mut self, _game: &mut G) {}
    fn buy<G: game::Game, const P: usize>(&mut self, game: &mut G) {
        while game.play_platinum::<P>() {}
        while game.play_gold::<P>() {}
        while game.play_silver::<P>() {}
        while game.play_copper::<P>() {}
        if game.colony_enabled() {
            if total_money::<G, P>(game) > 32 && game.buy_colony::<P>() {
                return;
            }
            if game.colony_in_supply() <= 6 && game.buy_province::<P>() {
                return;
            }
            if game.colony_in_supply() <= 5 && game.buy_duchy::<P>() {
                return;
            }
            if game.colony_in_supply() <= 2 && game.buy_estate::<P>() {
                return;
            }
            if game.buy_platinum::<P>() {
                return;
            }
            if game.colony_in_supply() <= 7 && game.buy_province::<P>() {
                return;
            }
            if game.buy_gold::<P>() {
                return;
            }
            if game.colony_in_supply() <= 6 && game.buy_duchy::<P>() {
                return;
            }
            game.buy_silver::<P>();
        } else {
            if total_money::<G, P>(game) > 18 && game.buy_province::<P>() {
                return;
            }
            if game.province_in_supply() <= 4 && game.buy_duchy::<P>() {
                return;
            }
            if game.province_in_supply() <= 2 && game.buy_estate::<P>() {
                return;
            }
            if game.buy_gold::<P>() {
                return;
            }
            if game.province_in_supply() <= 6 && game.buy_duchy::<P>() {
                return;
            }
            game.buy_silver::<P>();
        }
    }
}
