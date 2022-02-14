use crate::kingdom;
use crate::controller;
use std::marker::PhantomData;
use std::mem;
use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Copy,Clone)]
pub enum CardType {
    Province,
    Duchy,
    Estate,
    Gold,
    Silver,
    Copper,
    Curse,
    Village,
}

pub struct Game<K: kingdom::Kingdom, const N: usize> {
    province: u32,
    duchy: u32,
    estate: u32,
    gold: u32,
    silver: u32,
    copper: u32,
    curse: u32,

    kingdom: PhantomData<K>,
    players: [PersonalState; N],
    trash: Vec<CardType>,
}

pub struct PersonalState {
    deck: Vec<CardType>,
    discard: Vec<CardType>,
    hand: Vec<CardType>,
    action: u32,
    buy: u32,
    coin: u32,
}

impl PersonalState {
    pub fn make() -> PersonalState {
        PersonalState {
            deck: vec![],
            discard: vec![
                CardType::Estate,
                CardType::Estate,
                CardType::Estate,
                CardType::Copper,
                CardType::Copper,
                CardType::Copper,
                CardType::Copper,
                CardType::Copper,
                CardType::Copper,
                CardType::Copper,
            ],
            hand: vec![],
            action: 0,
            buy: 0,
            coin: 0,
        }
    }

    pub fn draw(&mut self) {
        if self.deck.len() == 0 {
            self.discard.shuffle(&mut thread_rng());
            mem::swap(&mut self.deck, &mut self.discard);
        }
        let card = self.deck.pop();
        match card {
            None => {},
            Some(x) => {self.hand.push(x);}
        }
    }

    pub fn turn_start(&mut self) {
        self.action = 1;
        self.buy = 1;
        self.coin = 0;
    }
}

impl<K: kingdom::Kingdom, const N: usize> Game<K, N> {
    pub fn make() -> Game<K, N> {
        let green_count = if N > 2 {12} else {8};
        let ret: Game<K, N> = Game {
            province: green_count,
            duchy: green_count,
            estate: green_count,
            gold: 30,
            silver: 40,
            copper: 46,
            curse: 10,
            kingdom: PhantomData,
            players: [(); N].map(|_| PersonalState::make()),
            trash: vec![],
        };
        ret
    }

    fn province_end(&mut self) -> bool {
        return self.province == 0;
    }

    fn pile_end(&mut self) -> bool {
        let mut empty_pile = 0;
        if self.duchy == 0 {
            empty_pile+=1;
        }
        if self.estate == 0 {
            empty_pile+=1;
        }
        if self.gold == 0 {
            empty_pile+=1;
        }
        if self.silver == 0 {
            empty_pile+=1;
        }
        if self.copper == 0 {
            empty_pile+=1;
        }
        if self.curse == 0 {
            empty_pile+=1;
        }
        empty_pile >= 3
    }

    pub fn run<T1: controller::Controller<K, N>, T2: controller::Controller<K, N>>(&mut self, t1: &mut T1, t2: &mut T2) -> u32 {
        for card in 0..5 {
            for player in 0..2 {
                self.players[player].draw();
            }
        }
        for _round in 1..100 {
            self.players[0].turn_start();
            t1.act();
            t1.buy(self);
            if self.province_end() || self.pile_end() {
                break;
            }
            self.players[1].turn_start();
            t2.act();
            t2.buy(self);
            if self.province_end() || self.pile_end() {
                break;
            }
        }
        0
    }
}
