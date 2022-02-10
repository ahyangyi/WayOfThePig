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
    pub fn make(n: u32) -> Game<K> {
        let green_count = if n > 2 {12} else {8};
        Game {
            province: green_count,
            duchy: green_count,
            estates: green_count,
            gold: 30,
            silver: 40,
            copper: 46,
            curse: 10,
            village: 10,
            kingdom: PhantomData,
        }
    }
    fn run(&self) {
    }
}
