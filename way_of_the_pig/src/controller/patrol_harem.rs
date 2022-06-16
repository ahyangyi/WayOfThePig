// Unoptimized strategy
use crate::card;
use crate::controller;
use crate::game;

pub struct Controller {}

fn total_money<G: game::Game, const P: usize>(game: &mut G) -> u32 {
    game.get_player::<P>().count_card(card::CardType::Platinum) * 5
        + game.get_player::<P>().count_card(card::CardType::Gold) * 3
        + game.get_player::<P>().count_card(card::CardType::Harem) * 2
        + game.get_player::<P>().count_card(card::CardType::Silver) * 2
        + game.get_player::<P>().count_card(card::CardType::Copper)
}

fn num_money<G: game::Game, const P: usize>(game: &mut G) -> u32 {
    game.get_player::<P>().count_card(card::CardType::Platinum)
        + game.get_player::<P>().count_card(card::CardType::Gold)
        + game.get_player::<P>().count_card(card::CardType::Harem)
        + game.get_player::<P>().count_card(card::CardType::Silver)
        + game.get_player::<P>().count_card(card::CardType::Copper)
}

impl Controller {
    pub fn make() -> Controller {
        Controller {}
    }
}

impl controller::Controller for Controller {
    fn act<G: game::Game, const P: usize>(&mut self, game: &mut G) {
        game.play_necropolis::<P>();
        game.play_patrol::<P>();
        game.play_patrol::<P>();
    }
    fn buy<G: game::Game, const P: usize>(&mut self, game: &mut G) {
        while game.play_platinum::<P>() {}
        while game.play_gold::<P>() {}
        while game.play_silver::<P>() {}
        while game.play_harem::<P>() {}
        while game.play_copper::<P>() {}
        if game.colony_enabled() {
            if game.buy_colony::<P>() {
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
            if game.get_player::<P>().count_card(card::CardType::Patrol) >= 2 && game.buy_harem::<P>() {
                return;
            }
            if game.buy_gold::<P>() {
                return;
            }
            if game.colony_in_supply() <= 6 && game.buy_duchy::<P>() {
                return;
            }
            if game.get_player::<P>().count_card(card::CardType::Patrol) * 4 < num_money::<G, P>(game) && game.buy_patrol::<P>() {
                return;
            }
            game.buy_silver::<P>();
        } else {
            if total_money::<G, P>(game) > 15 && game.buy_province::<P>() {
                return;
            }
            if game.buy_harem::<P>() {
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
            if game.get_player::<P>().count_card(card::CardType::Patrol) * 8
                < num_money::<G, P>(game)
                    + if game.get_player::<P>().count_card(card::CardType::Necropolis) == 1 {
                        8
                    } else {
                        0
                    }
                && game.buy_patrol::<P>()
            {
                return;
            }
            game.buy_silver::<P>();
        }
    }
}
