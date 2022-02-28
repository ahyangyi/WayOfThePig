use crate::kingdom;
use crate::controller;
use crate::pile;
use crate::pile::Pile;
use crate::card;
use std::marker::PhantomData;
use std::mem;
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

macro_rules! make_simple_buy_fn {
    ( $pile:ident, $f:ident, $p:expr ) => {
        fn $f<const P: usize>(&mut self) -> bool {
            if !self.$pile.enabled() ||
                self.players[P].buy == 0 ||
                self.players[P].coin < $p {
                return false;
            }
            let card = self.$pile.top();
            match card {
                None => false,
                Some(x) => {
                    self.$pile.pop();
                    self.players[P].buy -= 1;
                    self.players[P].coin -= 8;
                    self.players[P].gain(x);
                    self.players[P].deck_stats[x as usize] += 1;
                    true
                }
            }
        }
    };
}

macro_rules! make_simple_play_fn {
    ( $card:ident, $f:ident ) => {
        pub fn $f(&mut self) -> bool {
            if self.hand[CardType::$card as usize] == 0 {
                return false;
            }
            self.hand[CardType::$card as usize] -= 1;
            self.play.push(CardType::$card);
            self.coin += 3;
            true
        }
    };
}


#[derive(Copy,Clone,PartialEq,Debug,FromPrimitive)]
pub enum CardType {
    // Base Cards
    Province,
    Duchy,
    Estate,
    Gold,
    Silver,
    Copper,
    Curse,

    // Colony
    Colony,
    Platinum,

    // Shelter
    OvergrownEstate,
    Hovel,
    Necropolis,

    // Base Set
    Village,
    Smithy,
    Militia,
    Market,

    // Intrigue
    Upgrade,
    Patrol,
    Harem,

    // Hinterland
    Oasis,
    SpiceMerchant,
    Stables,

    // Nocturne
    FaithfulHound,
}

const CARDTYPES : usize = 23;

pub trait GameState {
    // buy APIs
    fn buy_province<const P: usize>(&mut self) -> bool;
    fn buy_duchy<const P: usize>(&mut self) -> bool;
    fn buy_estate<const P: usize>(&mut self) -> bool;
    fn buy_gold<const P: usize>(&mut self) -> bool;
    fn buy_silver<const P: usize>(&mut self) -> bool;
    fn buy_copper<const P: usize>(&mut self) -> bool;
    fn buy_curse<const P: usize>(&mut self) -> bool;

    fn buy_colony<const P: usize>(&mut self) -> bool;
    fn buy_platinum<const P: usize>(&mut self) -> bool;

    fn buy_smithy<const P: usize>(&mut self) -> bool;
    fn buy_patrol<const P: usize>(&mut self) -> bool;

    // supply inspection
    fn province_in_supply(&self) -> u8;
    fn colony_in_supply(&self) -> u8;
    fn colony_enabled(&self) -> bool;

    fn get_player<const P: usize>(&mut self) -> &mut PersonalState;
}

pub struct Game<K: kingdom::Kingdom, const N: usize> {
    province: pile::province::Pile,
    duchy: pile::duchy::Pile,
    estate: pile::estate::Pile,
    gold: pile::gold::Pile,
    silver: pile::silver::Pile,
    copper: pile::copper::Pile,
    curse: pile::curse::Pile,

    colony: pile::colony::Pile,
    platinum: pile::platinum::Pile,

    smithy: pile::smithy::Pile,
    patrol: pile::patrol::Pile,

    kingdom: PhantomData<K>,
    players: [PersonalState; N],
    trash: Vec<CardType>,
}

pub struct PersonalState {
    deck: Vec<CardType>,
    discard: Vec<CardType>,
    hand: [u32; CARDTYPES],
    play: Vec<CardType>,
    deck_stats: [u32; CARDTYPES],
    action: u32,
    buy: u32,
    pub coin: u32,
}

impl PersonalState {
    pub fn make() -> PersonalState {
        let mut ret = PersonalState {
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
            hand: [0; CARDTYPES],
            play: vec![],
            deck_stats: [0; CARDTYPES],
            action: 0,
            buy: 0,
            coin: 0,
        };
        ret.deck_stats[CardType::Copper as usize] = 7;
        ret.deck_stats[CardType::Estate as usize] = 3;
        ret
    }

    #[inline]
    pub fn draw_to(&mut self) -> Option<CardType> {
        if self.deck.len() == 0 {
            self.discard.shuffle(&mut thread_rng());
            mem::swap(&mut self.deck, &mut self.discard);
        }
        self.deck.pop()
    }

    pub fn draw(&mut self) {
        let card = self.draw_to();
        match card {
            None => {},
            Some(x) => {self.hand[x as usize] += 1;}
        }
    }

    #[inline]
    pub fn gain(&mut self, c: CardType) {
        self.discard.push(c);
    }

    pub fn turn_start(&mut self) {
        self.action = 1;
        self.buy = 1;
        self.coin = 0;
    }

    pub fn clean_up(&mut self) {
        for i in 0..CARDTYPES {
            for _j in 0..self.hand[i] {
                self.discard.push(FromPrimitive::from_usize(i).unwrap());
            }
            self.hand[i] = 0;
        }
        self.discard.append(&mut self.play);
        for _card in 0..5 {
            self.draw();
        }
    }

    make_simple_play_fn!(Gold, play_gold);

    pub fn play_platinum(&mut self) -> bool {
        if self.hand[CardType::Platinum as usize] == 0 {
            return false;
        }
        self.hand[CardType::Platinum as usize] -= 1;
        self.play.push(CardType::Platinum);
        self.coin += 5;
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

    pub fn play_smithy(&mut self) -> bool {
        if self.hand[CardType::Smithy as usize] == 0 || self.action == 0 {
            return false;
        }
        self.hand[CardType::Smithy as usize] -= 1;
        self.play.push(CardType::Smithy);
        self.draw();
        self.draw();
        self.draw();
        true
    }

    pub fn play_patrol(&mut self) -> bool {
        if self.hand[CardType::Patrol as usize] == 0 || self.action == 0 {
            return false;
        }
        self.hand[CardType::Patrol as usize] -= 1;
        self.play.push(CardType::Patrol);
        self.draw();
        self.draw();
        self.draw();

        let mut m : Vec<CardType> = vec![];
        for _i in 0..4 {
            let card = self.draw_to();
            match card {
                None => {break;},
                Some(x) => {
                    if x == CardType::Province || x == CardType::Duchy || x == CardType::Estate {
                        self.hand[x as usize] += 1;
                    } else {
                        m.push(x);
                    }
                }
            }
        }

        // FIXME: let the player to decide the order
        self.deck.append(&mut m);
        true
    }

    // only guarantees meaningful results at game end
    pub fn total_final_vp(&self) -> u32 {
        self.count_card_static::<{CardType::Colony as usize}>() * 10 +
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
        self.deck_stats[c as usize]
    }

    pub fn count_card_static<const C: usize>(&self) -> u32 {
        self.deck_stats[C]
    }
}

impl<K: kingdom::Kingdom, const N: usize> Game<K, N> {
    pub fn make() -> Game<K, N> {
        let ret: Game<K, N> = Game {
            province: pile::province::Pile::make::<N>(),
            duchy: pile::duchy::Pile::make::<N>(),
            estate: pile::estate::Pile::make::<N>(),
            gold: pile::gold::Pile::make::<N>(),
            silver: pile::silver::Pile::make::<N>(),
            copper: pile::copper::Pile::make::<N>(),
            curse: pile::curse::Pile::make::<N>(),
            colony: pile::colony::Pile::make::<N>(),
            platinum: pile::platinum::Pile::make::<N>(),
            smithy: pile::smithy::Pile::make::<N>(),
            patrol: pile::patrol::Pile::make::<N>(),
            kingdom: PhantomData,
            players: [(); N].map(|_| PersonalState::make()),
            trash: vec![],
        };
        ret
    }

    fn province_end(&self) -> bool {
        self.province_in_supply() == 0
    }
    
    fn colony_end(&self) -> bool {
        false
    }

    fn pile_end(&self) -> bool {
        let mut empty_pile = 0;
        if self.duchy.remaining_cards() == 0 {
            empty_pile+=1;
        }
        if self.estate.remaining_cards() == 0 {
            empty_pile+=1;
        }
        if self.gold.remaining_cards() == 0 {
            empty_pile+=1;
        }
        if self.silver.remaining_cards() == 0 {
            empty_pile+=1;
        }
        if self.copper.remaining_cards() == 0 {
            empty_pile+=1;
        }
        if self.curse.remaining_cards() == 0 {
            empty_pile+=1;
        }
        let end_condition = if N >= 4 { 3 } else { 4 };
        empty_pile >= end_condition
    }

    fn end(&self) -> bool {
        self.province_end() || self.colony_end() || self.pile_end()
    }

    pub fn run<T1: controller::Controller, T2: controller::Controller>(&mut self, t1: &mut T1, t2: &mut T2) -> [u32; 2] {
        for player in 0..2 {
            for _card in 0..5 {
                self.players[player].draw();
            }
        }
        let mut break_pos : u32 = 0;
        for _round in 1..100 {
            self.players[0].turn_start();
            t1.act::<Game<K, N>, 0>(self);
            t1.buy::<Game<K, N>, 0>(self);
            if self.end() {
                break;
            }
            self.players[0].clean_up();
            self.players[1].turn_start();
            t2.act::<Game<K, N>, 1>(self);
            t2.buy::<Game<K, N>, 1>(self);
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

    pub fn run_random<T1: controller::Controller, T2: controller::Controller>(&mut self, t1: &mut T1, t2: &mut T2) -> [u32; 2] {
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
    make_simple_buy_fn!(province, buy_province, 8);
    make_simple_buy_fn!(duchy, buy_duchy, 5);
    make_simple_buy_fn!(estate, buy_estate, 2);
    make_simple_buy_fn!(gold, buy_gold, 6);
    make_simple_buy_fn!(silver, buy_silver, 3);
    make_simple_buy_fn!(copper, buy_copper, 0);
    make_simple_buy_fn!(curse, buy_curse, 0);

    make_simple_buy_fn!(colony, buy_colony, 11);
    make_simple_buy_fn!(platinum, buy_platinum, 9);

    make_simple_buy_fn!(smithy, buy_smithy, 4);
    make_simple_buy_fn!(patrol, buy_patrol, 5);

    fn get_player<const P: usize>(&mut self) -> &mut PersonalState {
        &mut self.players[P]
    }

    #[inline]
    fn province_in_supply(&self) -> u8 {
        self.province.remaining_cards()
    }

    #[inline]
    fn colony_in_supply(&self) -> u8 {
        self.colony.remaining_cards()
    }

    #[inline]
    fn colony_enabled(&self) -> bool {
        self.colony.enabled()
    }
}
