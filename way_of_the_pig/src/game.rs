use crate::card;
use crate::card::{Card, CardType};
use crate::controller;
use crate::kingdom;
use crate::observer;
use crate::pile;
use crate::pile::Pile;
use num_traits::FromPrimitive;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::mem;

macro_rules! make_simple_buy_fn {
    ( $pile:ident, $f:ident ) => {
        fn $f<const P: usize>(&mut self) -> bool {
            if !self.$pile.enabled() || self.players[P].buy == 0 {
                return false;
            }
            let card = self.$pile.top();
            match card {
                None => false,
                Some(x) => {
                    if self.players[P].coin < card::static_price(x) {
                        return false;
                    }
                    self.$pile.pop();
                    self.players[P].buy -= 1;
                    self.players[P].coin -= card::static_price(x);
                    self.players[P].gain(x);
                    self.players[P].deck_stats[x as usize] += 1;
                    true
                }
            }
        }
    };
}

macro_rules! make_simple_play_fn {
    ( $card:ident, $c:ident, $f:ident ) => {
        fn $f<const P: usize>(&mut self) -> bool {
            if self.players[P].hand[CardType::$card as usize] == 0 {
                return false;
            }
            self.players[P].hand[CardType::$card as usize] -= 1;
            self.players[P].play.push(CardType::$card);
            card::$c::Card::play::<Self, P>(self);
            true
        }
    };
}

macro_rules! count_empty_pile {
    ( $self:ident, $cnt:ident; $c:ident ) => {
        if $self.$c.enabled() && $self.$c.remaining_cards() == 0 {
            $cnt += 1;
        }
    };
    ( $self:ident, $cnt:ident; $c:ident, $($tail:ident),* ) => {
        count_empty_pile!($self, $cnt; $c);
        count_empty_pile!($self, $cnt; $($tail),*);
    };
}

const CARDTYPES: usize = 23;

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
    fn buy_village<const P: usize>(&mut self) -> bool;
    fn buy_patrol<const P: usize>(&mut self) -> bool;
    fn buy_harem<const P: usize>(&mut self) -> bool;

    // play APIs
    fn play_gold<const P: usize>(&mut self) -> bool;
    fn play_silver<const P: usize>(&mut self) -> bool;
    fn play_copper<const P: usize>(&mut self) -> bool;

    fn play_platinum<const P: usize>(&mut self) -> bool;

    fn play_smithy<const P: usize>(&mut self) -> bool;
    fn play_village<const P: usize>(&mut self) -> bool;
    fn play_patrol<const P: usize>(&mut self) -> bool;
    fn play_harem<const P: usize>(&mut self) -> bool;

    // supply inspection
    fn province_in_supply(&self) -> u8;
    fn colony_in_supply(&self) -> u8;
    fn colony_enabled(&self) -> bool;

    fn get_player<const P: usize>(&mut self) -> &mut PersonalState;

    // hooked effects
    fn add_coin<const P: usize>(&mut self, c: u32);
    fn draw_to<const P: usize>(&mut self) -> Option<CardType>;
    fn draw<const P: usize>(&mut self);
}

pub struct Game<'a, K: kingdom::Kingdom, O: observer::Observer, const N: usize> {
    province: pile::province::Pile,
    duchy: pile::duchy::Pile,
    estate: pile::estate::Pile,
    gold: pile::gold::Pile,
    silver: pile::silver::Pile,
    copper: pile::copper::Pile,
    curse: pile::curse::Pile,

    colony: K::ColonyPile,
    platinum: K::PlatinumPile,

    smithy: pile::smithy::Pile,
    village: pile::village::Pile,
    patrol: pile::patrol::Pile,
    harem: pile::harem::Pile,

    players: [PersonalState; N],
    trash: Vec<CardType>,

    observer: &'a mut O,
}

pub struct PersonalState {
    pub deck: Vec<CardType>,
    discard: Vec<CardType>,
    pub hand: [u32; CARDTYPES],
    play: Vec<CardType>,
    deck_stats: [u32; CARDTYPES],
    pub action: u32,
    pub buy: u32,
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
            None => {}
            Some(x) => {
                self.hand[x as usize] += 1;
            }
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

    // only guarantees meaningful results at game end
    pub fn total_final_vp(&self) -> u32 {
        self.count_card_static::<{ CardType::Colony as usize }>() * 10
            + self.count_card_static::<{ CardType::Province as usize }>() * 6
            + self.count_card_static::<{ CardType::Duchy as usize }>() * 3
            + self.count_card_static::<{ CardType::Estate as usize }>() * 1
            + self.count_card_static::<{ CardType::Harem as usize }>() * 2
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

impl<'a, K: kingdom::Kingdom, O: observer::Observer, const N: usize> Game<'a, K, O, N> {
    pub fn make(o: &'a mut O) -> Game<'a, K, O, N> {
        let ret: Game<'a, K, O, N> = Game {
            province: pile::province::Pile::make::<N>(),
            duchy: pile::duchy::Pile::make::<N>(),
            estate: pile::estate::Pile::make::<N>(),
            gold: pile::gold::Pile::make::<N>(),
            silver: pile::silver::Pile::make::<N>(),
            copper: pile::copper::Pile::make::<N>(),
            curse: pile::curse::Pile::make::<N>(),
            colony: K::ColonyPile::make::<N>(),
            platinum: K::PlatinumPile::make::<N>(),
            smithy: pile::smithy::Pile::make::<N>(),
            village: pile::village::Pile::make::<N>(),
            patrol: pile::patrol::Pile::make::<N>(),
            harem: pile::harem::Pile::make::<N>(),
            players: [(); N].map(|_| PersonalState::make()),
            trash: vec![],
            observer: o,
        };
        ret
    }

    #[inline]
    fn province_end(&self) -> bool {
        self.province_in_supply() == 0
    }

    #[inline]
    fn colony_end(&self) -> bool {
        self.colony_enabled() && self.colony_in_supply() == 0
    }

    #[inline]
    fn pile_end(&self) -> bool {
        let mut empty_pile = 0;
        count_empty_pile!(self, empty_pile; duchy, estate, gold, silver, copper, curse, smithy, village, harem, patrol);
        let end_condition = if N >= 4 { 3 } else { 4 };
        empty_pile >= end_condition
    }

    fn end(&self) -> bool {
        self.province_end() || self.colony_end() || self.pile_end()
    }

    pub fn run<T1: controller::Controller, T2: controller::Controller>(&mut self, t1: &mut T1, t2: &mut T2) {
        for player in 0..2 {
            for _card in 0..5 {
                self.players[player].draw();
            }
        }
        let mut break_pos: u32 = 0;
        for round in 0..100 {
            self.observer.notify_turn::<0>(round);
            self.players[0].turn_start();
            t1.act::<Game<K, O, N>, 0>(self);
            t1.buy::<Game<K, O, N>, 0>(self);
            if self.end() {
                break;
            }
            self.players[0].clean_up();
            self.observer.notify_turn::<1>(round);
            self.players[1].turn_start();
            t2.act::<Game<K, O, N>, 1>(self);
            t2.buy::<Game<K, O, N>, 1>(self);
            if self.end() {
                break_pos = 1;
                break;
            }
            self.players[1].clean_up();
        }
        let vp_0 = self.players[0].total_final_vp();
        let vp_1 = self.players[1].total_final_vp();
        let ret = if vp_0 > vp_1 {
            [0, 1]
        } else if vp_0 < vp_1 {
            [1, 0]
        } else if break_pos == 0 {
            [0, 1]
        } else {
            [0, 0]
        };
        self.observer.notify_result_2(ret);
    }
}

impl<K: kingdom::Kingdom, O: observer::Observer, const N: usize> GameState for Game<'_, K, O, N> {
    make_simple_buy_fn!(province, buy_province);
    make_simple_buy_fn!(duchy, buy_duchy);
    make_simple_buy_fn!(estate, buy_estate);
    make_simple_buy_fn!(gold, buy_gold);
    make_simple_buy_fn!(silver, buy_silver);
    make_simple_buy_fn!(copper, buy_copper);
    make_simple_buy_fn!(curse, buy_curse);

    make_simple_buy_fn!(colony, buy_colony);
    make_simple_buy_fn!(platinum, buy_platinum);

    make_simple_buy_fn!(smithy, buy_smithy);
    make_simple_buy_fn!(village, buy_village);
    make_simple_buy_fn!(harem, buy_harem);
    make_simple_buy_fn!(patrol, buy_patrol);

    make_simple_play_fn!(Gold, gold, play_gold);
    make_simple_play_fn!(Silver, silver, play_silver);
    make_simple_play_fn!(Copper, copper, play_copper);

    make_simple_play_fn!(Platinum, platinum, play_platinum);

    make_simple_play_fn!(Smithy, smithy, play_smithy);
    make_simple_play_fn!(Village, village, play_village);

    make_simple_play_fn!(Harem, harem, play_harem);
    make_simple_play_fn!(Patrol, patrol, play_patrol);

    fn get_player<const P: usize>(&mut self) -> &mut PersonalState {
        &mut self.players[P]
    }

    #[inline]
    fn add_coin<const P: usize>(&mut self, c: u32) {
        self.players[P].coin += c;
        self.observer.add_coin::<P>(c);
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

    #[inline]
    fn draw_to<const P: usize>(&mut self) -> Option<CardType> {
        self.players[P].draw_to()
    }

    #[inline]
    fn draw<const P: usize>(&mut self) {
        self.players[P].draw()
    }
}
