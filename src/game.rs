use crate::kingdom;
use std::marker::PhantomData;

pub struct Game<K: kingdom::Kingdom> {
    province: u32,
    duchy: u32,
    estates: u32,
    gold: u32,
    silver: u32,
    copper: u32,
    curse: u32,

    village: u32,

    kingdom: PhantomData<K>,
}

impl<K: kingdom::Kingdom> Game<K> {
    pub fn make() -> Game<K> {
        Game {
            province: 8,
            duchy: 8,
            estates: 8,
            gold: 8,
            silver: 8,
            copper: 8,
            curse: 10,
            village: 10,
            kingdom: PhantomData,
        }
    }
    fn run(&self) {
    }
}
