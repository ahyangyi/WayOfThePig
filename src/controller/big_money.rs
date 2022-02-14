// An implementation of WanderingWinder's "Big Money Ultimate", which attempts to buy the first
// possible entry:
//   Buy province if total money > 18
//   Buy duchy if remaining province <= 4
//   Buy estate if remaining province <= 2
//   Buy gold
//   Buy Duchy if remaining province <= 6
//   Buy silver
use crate::kingdom;
use crate::game;
use crate::controller;
use std::marker::PhantomData;

pub struct BigMoneyController<K: kingdom::Kingdom> {
    kingdom: PhantomData<K>,
}

fn total_money() -> u32 {
    1
}

impl<K: kingdom::Kingdom> controller::Controller<K> for BigMoneyController<K> {
    fn act(&mut self) {
    }
    fn buy(&mut self, game: &mut game::Game<K, 2>, hand: &mut game::PersonalState) {
        if total_money() > 18 {

        }
        // game.buy_gold() || game.buy_silver();
    }
}
