use crate::kingdom;
use crate::controller;
use std::marker::PhantomData;
use std::mem;
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Copy,Clone,PartialEq,Debug,FromPrimitive)]
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

pub trait GameState {
    fn buy_province<const P: usize>(&mut self) -> bool;
    fn buy_duchy<const P: usize>(&mut self) -> bool;
    fn buy_estate<const P: usize>(&mut self) -> bool;
    fn buy_gold<const P: usize>(&mut self) -> bool;
    fn buy_silver<const P: usize>(&mut self) -> bool;
    fn get_player<const P: usize>(&mut self) -> &mut PersonalState;
    fn province_in_supply(&self) -> u32;
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
    hand: [u32; 7],
    play: Vec<CardType>,
    deckStats: [u32; 7],
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
            hand: [0; 7],
            play: vec![],
            deckStats: [0, 0, 3, 0, 0, 7, 0],
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
            Some(x) => {self.hand[x as usize] += 1;}
        }
    }

    pub fn turn_start(&mut self) {
        self.action = 1;
        self.buy = 1;
        self.coin = 0;
    }

    pub fn clean_up(&mut self) {
        for i in 0..7 {
            for _j in 0..self.hand[i] {
                self.discard.push(FromPrimitive::from_usize(i).unwrap());
            }
            self.hand[i] = 0;
        }
        while self.play.len() > 0 {
            self.discard.push(self.play.pop().unwrap());
        }
        for _card in 0..5 {
            self.draw();
        }
    }

    pub fn play_gold(&mut self) -> bool {
        if self.hand[CardType::Gold as usize] == 0 {
            return false;
        }
        self.hand[CardType::Gold as usize] -= 1;
        self.play.push(CardType::Gold);
        self.coin += 3;
        true
    }

    pub fn play_silver(&mut self) -> bool {
        if self.hand[CardType::Silver as usize] == 0 {
            return false;
        }
        self.hand[CardType::Silver as usize] -= 1;
        self.play.push(CardType::Silver);
        self.coin += 2;
        true
    }

    pub fn play_copper(&mut self) -> bool {
        if self.hand[CardType::Copper as usize] == 0 {
            return false;
        }
        self.hand[CardType::Copper as usize] -= 1;
        self.play.push(CardType::Copper);
        self.coin += 1;
        true
    }

    // only guarantees meaningful results at game end
    pub fn total_final_vp(&self) -> u32 {
        self.count_card_static::<{CardType::Province as usize}>() * 6 +
        self.count_card_static::<{CardType::Duchy as usize}>() * 3 +
        self.count_card_static::<{CardType::Estate as usize}>() * 1
    }

    pub fn get_action(&self) -> u32 {
        self.action
    }

    pub fn get_buy(&self) -> u32 {
        self.buy
    }

    pub fn get_coin(&self) -> u32 {
        self.coin
    }

    pub fn count_card(&self, c: CardType) -> u32 {
        self.deckStats[c as usize]
    }

    pub fn count_card_static<const C: usize>(&self) -> u32 {
        self.deckStats[C]
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

    fn province_end(&self) -> bool {
        self.province == 0
    }
    
    fn colony_end(&self) -> bool {
        false
    }

    fn pile_end(&self) -> bool {
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

    fn end(&self) -> bool {
        self.province_end() || self.colony_end() || self.pile_end()
    }

    pub fn run<T1: controller::Controller<Game<K, N>>, T2: controller::Controller<Game<K, N>>>(&mut self, t1: &mut T1, t2: &mut T2) -> [u32; 2] {
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
            if self.end() {
                break;
            }
            self.players[0].clean_up();
            self.players[1].turn_start();
            t2.act();
            t2.buy::<1>(self);
            if self.end() {
                break_pos = 1;
                break;
            }
            self.players[1].clean_up();
        }
        let vp_0 = self.players[0].total_final_vp();
        let vp_1 = self.players[1].total_final_vp();
        if vp_0 > vp_1 {
            [0, 1]
        } else if vp_0 < vp_1 {
            [1, 0]
        } else if break_pos == 0 {
            [0, 1]
        } else {
            [0, 0]
        }
    }

    pub fn run_random<T1: controller::Controller<Game<K, N>>, T2: controller::Controller<Game<K, N>>>(&mut self, t1: &mut T1, t2: &mut T2) -> [u32; 2] {
        let mut rng = rand::thread_rng();
        let scheme = rng.gen_range(0..2);

        if scheme == 0 {
            self.run(t1, t2)
        } else {
            let [r2, r1] = self.run(t2, t1);
            [r1, r2]
        }
    }
}

impl<K: kingdom::Kingdom, const N: usize> GameState for Game<K, N> {
    fn buy_province<const P: usize>(&mut self) -> bool {
        if self.province == 0 || self.players[P].buy == 0 || self.players[P].coin < 8 {
            return false;
        }
        self.province -= 1;
        self.players[P].buy -= 1;
        self.players[P].coin -= 8;
        self.players[P].discard.push(CardType::Province);
        self.players[P].deckStats[CardType::Province as usize] += 1;
        true
    }

    fn buy_duchy<const P: usize>(&mut self) -> bool {
        if self.duchy == 0 || self.players[P].buy == 0 || self.players[P].coin < 5 {
            return false;
        }
        self.duchy -= 1;
        self.players[P].buy -= 1;
        self.players[P].coin -= 5;
        self.players[P].discard.push(CardType::Duchy);
        self.players[P].deckStats[CardType::Duchy as usize] += 1;
        true
    }

    fn buy_estate<const P: usize>(&mut self) -> bool {
        if self.estate == 0 || self.players[P].buy == 0 || self.players[P].coin < 2 {
            return false;
        }
        self.estate -= 1;
        self.players[P].buy -= 1;
        self.players[P].coin -= 2;
        self.players[P].discard.push(CardType::Estate);
        self.players[P].deckStats[CardType::Estate as usize] += 1;
        true
    }

    fn buy_gold<const P: usize>(&mut self) -> bool {
        if self.gold == 0 || self.players[P].buy == 0 || self.players[P].coin < 6 {
            return false;
        }
        self.gold -= 1;
        self.players[P].buy -= 1;
        self.players[P].coin -= 6;
        self.players[P].discard.push(CardType::Gold);
        self.players[P].deckStats[CardType::Gold as usize] += 1;
        true
    }

    fn buy_silver<const P: usize>(&mut self) -> bool {
        if self.silver == 0 || self.players[P].buy == 0 || self.players[P].coin < 3 {
            return false;
        }
        self.silver -= 1;
        self.players[P].buy -= 1;
        self.players[P].coin -= 3;
        self.players[P].discard.push(CardType::Silver);
        self.players[P].deckStats[CardType::Silver as usize] += 1;
        true
    }

    fn get_player<const P: usize>(&mut self) -> &mut PersonalState {
        &mut self.players[P]
    }

    fn province_in_supply(&self) -> u32 {
        self.province
    }
}
