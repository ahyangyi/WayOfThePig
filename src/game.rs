use crate::kingdom;
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

    village: u32,

    kingdom: PhantomData<K>,
    players: [PersonalState; 2],
    trash: Vec<CardType>,
}

pub struct PersonalState {
    deck: Vec<CardType>,
    discard: Vec<CardType>,
    hand: Vec<CardType>,
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
}

impl<K: kingdom::Kingdom, const N: usize> Game<K, N> {
    pub fn make() -> Game<K, N> {
        let green_count = if N > 2 {12} else {8};
        Game {
            province: green_count,
            duchy: green_count,
            estate: green_count,
            gold: 30,
            silver: 40,
            copper: 46,
            curse: 10,
            village: 10,
            kingdom: PhantomData,
            players: [PersonalState::make(), PersonalState::make()],
            trash: vec![],
        }
    }

    pub fn run(&mut self) -> u32 {
        for card in 0..5 {
            self.players[0].draw();
            self.players[1].draw();
        }
        for _round in 1..100 {

        }
        0
    }
}
