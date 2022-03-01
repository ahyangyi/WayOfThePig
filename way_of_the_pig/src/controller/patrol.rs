// Unoptimized strategy
use crate::card;
use crate::game;
use crate::controller;

pub struct Controller {
}

fn total_money<G: game::GameState, const P: usize>(game: &mut G) -> u32 {
    game.get_player::<P>().count_card(card::CardType::Gold) * 3 +
    game.get_player::<P>().count_card(card::CardType::Silver) * 2 +
    game.get_player::<P>().count_card(card::CardType::Copper) * 1
}

fn num_money<G: game::GameState, const P: usize>(game: &mut G) -> u32 {
    game.get_player::<P>().count_card(card::CardType::Gold) +
    game.get_player::<P>().count_card(card::CardType::Silver) +
    game.get_player::<P>().count_card(card::CardType::Copper)
}

impl Controller {
    pub fn make() -> Controller{
        Controller {}
    }
}

impl controller::Controller for Controller {
    fn act<G: game::GameState, const P: usize>(&mut self, game: &mut G) {
        game.play_patrol::<P>();
    }
    fn buy<G: game::GameState, const P: usize>(&mut self, game: &mut G) {
        while game.play_platinum::<P>() {}
        while game.play_gold::<P>() {}
        while game.play_silver::<P>() {}
        while game.play_copper::<P>() {}
        if game.colony_enabled() {
            if game.buy_colony::<P>() {
                return;
            } else if game.colony_in_supply() <= 6 && game.buy_province::<P>() {
                return;
            } else if game.colony_in_supply() <= 5 && game.buy_duchy::<P>() {
                return;
            } else if game.colony_in_supply() <= 2 && game.buy_estate::<P>() {
                return;
            } else if game.buy_platinum::<P>() {
                return;
            } else if game.colony_in_supply() <= 7 && game.buy_province::<P>() {
                return;
            } else if game.buy_gold::<P>() {
                return;
            } else if game.colony_in_supply() <= 6 && game.buy_duchy::<P>() {
                return;
            } else if game.get_player::<P>().count_card(card::CardType::Patrol) * 8 < num_money::<G, P>(game) && game.buy_patrol::<P>() {
                return;
            } else {
                game.buy_silver::<P>();
            }
        } else {
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
            } else if game.get_player::<P>().count_card(card::CardType::Patrol) * 8 < num_money::<G, P>(game) && game.buy_patrol::<P>() {
                return;
            } else {
                game.buy_silver::<P>();
            }
        }
    }
}
