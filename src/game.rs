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
    pub province: u32,
    duchy: u32,
    estate: u32,
    gold: u32,
    silver: u32,
    copper: u32,
    curse: u32,

    kingdom: PhantomData<K>,
    pub players: [PersonalState; N],
    trash: Vec<CardType>,
}

pub struct PersonalState {
    deck: Vec<CardType>,
    discard: Vec<CardType>,
    hand: Vec<CardType>,
    pub action: u32,
    pub buy: u32,
    pub coin: u32,
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
        for player in 0..2 {
            for _card in 0..5 {
                self.players[player].draw();
            }
        }
        let mut break_pos : u32 = 0;
        for _round in 1..100 {
            self.players[0].turn_start();
            t1.act();
            t1.buy::<0>(self);
            if self.province_end() || self.pile_end() {
                break;
            }
            self.players[1].turn_start();
            t2.act();
            t2.buy::<1>(self);
            if self.province_end() || self.pile_end() {
                break_pos = 1;
                break;
            }
        }
        let mut vp_0 = 0;
        let mut vp_1 = 0;
        let ret:u32 = if vp_0 > vp_1 {0} else {1};
        ret
    }

    pub fn buy_province<const P: usize>(&mut self) -> bool {
        if self.province == 0 || self.players[P].buy == 0 || self.players[P].coin < 8 {
            return false;
        }
        self.province -= 1;
        self.players[P].buy -= 1;
        self.players[P].coin -= 8;
        self.players[P].discard.push(CardType::Province);
        true
    }

    pub fn buy_duchy<const P: usize>(&mut self) -> bool {
        if self.duchy == 0 || self.players[P].buy == 0 || self.players[P].coin < 5 {
            return false;
        }
        self.duchy -= 1;
        self.players[P].buy -= 1;
        self.players[P].coin -= 5;
        self.players[P].discard.push(CardType::Duchy);
        true
    }

    pub fn buy_estate<const P: usize>(&mut self) -> bool {
        if self.estate == 0 || self.players[P].buy == 0 || self.players[P].coin < 2 {
            return false;
        }
        self.estate -= 1;
        self.players[P].buy -= 1;
        self.players[P].coin -= 2;
        self.players[P].discard.push(CardType::Estate);
        true
    }

    pub fn buy_gold<const P: usize>(&mut self) -> bool {
        if self.gold == 0 || self.players[P].buy == 0 || self.players[P].coin < 6 {
            return false;
        }
        self.gold -= 1;
        self.players[P].buy -= 1;
        self.players[P].coin -= 6;
        self.players[P].discard.push(CardType::Gold);
        true
    }

    pub fn buy_silver<const P: usize>(&mut self) -> bool {
        if self.silver == 0 || self.players[P].buy == 0 || self.players[P].coin < 3 {
            return false;
        }
        self.silver -= 1;
        self.players[P].buy -= 1;
        self.players[P].coin -= 3;
        self.players[P].discard.push(CardType::Silver);
        true
    }
}
