use crate::kingdom;
use std::marker::PhantomData;

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

pub struct Game<K: kingdom::Kingdom> {
    province: u32,
    duchy: u32,
    estate: u32,
    gold: u32,
    silver: u32,
    copper: u32,
    curse: u32,

    village: u32,

    kingdom: PhantomData<K>,
    decks: [Vec<CardType>; 2],
    discards: [Vec<u32>; 2],
    trash: Vec<u32>,
}

impl<K: kingdom::Kingdom> Game<K> {
    pub fn make(n: u32) -> Game<K> {
        let green_count = if n > 2 {12} else {8};
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
            decks: [vec![
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
            ], vec![
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
            ]],
            discards: [vec![], vec![]],
            trash: vec![],
        }
    }
    fn run(&self) {
    }
}
