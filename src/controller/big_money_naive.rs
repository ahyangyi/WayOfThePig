// A weaker Big Money bot than "Big Money Ultimate". Strategy:
//   Buy province if total money > 18
//   Buy gold
//   Buy silver
use crate::kingdom;
use crate::game;
use crate::controller;
use crate::game::GameState;
use std::marker::PhantomData;

pub struct BigMoneyController<K: kingdom::Kingdom, const N: usize> {
    kingdom: PhantomData<K>,
}

fn total_money<K: kingdom::Kingdom, const N: usize, const P: usize>(game: &mut game::Game<K, N>) -> u32 {
    game.players[P].count_card(game::CardType::Gold) * 3 +
    game.players[P].count_card(game::CardType::Silver) * 2 +
    game.players[P].count_card(game::CardType::Copper) * 1
}

impl<K: kingdom::Kingdom, const N: usize> BigMoneyController<K, N> {
    pub fn make() -> BigMoneyController<K, N> {
        BigMoneyController {
            kingdom: PhantomData,
        }
    }
}

impl<K: kingdom::Kingdom, const N: usize> controller::Controller<K, N> for BigMoneyController<K, N> {
    fn act(&mut self) {
    }
    fn buy<const P: usize>(&mut self, game: &mut game::Game<K, N>) {
        while game.players[P].play_gold() {}
        while game.players[P].play_silver() {}
        while game.players[P].play_copper() {}
        if total_money::<K, N, P>(game) > 18 && game.buy_province::<P>() {
            return;
        } else if game.buy_gold::<P>() {
            return;
        } else {
            game.buy_silver::<P>();
        }
    }
}
