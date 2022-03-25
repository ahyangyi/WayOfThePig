use crate::card;
use crate::card::{Card, CardType};
use crate::controller;
use crate::game::run_round::RoundPlayer;
use crate::kingdom;
use crate::observer;
use crate::pile;
use crate::pile::Pile;
use num_traits::FromPrimitive;
use rand::seq::SliceRandom;
use std::mem;

mod run_round;
mod table;

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
            card::$c::Card::on_play::<Self, P>(self);
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
    type Observer: observer::Observer;
    type Table: RoundPlayer + table::Table;

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

    fn buy_faithful_hound<const P: usize>(&mut self) -> bool;

    // play APIs
    fn play_gold<const P: usize>(&mut self) -> bool;
    fn play_silver<const P: usize>(&mut self) -> bool;
    fn play_copper<const P: usize>(&mut self) -> bool;

    fn play_platinum<const P: usize>(&mut self) -> bool;

    fn play_necropolis<const P: usize>(&mut self) -> bool;

    fn play_smithy<const P: usize>(&mut self) -> bool;
    fn play_village<const P: usize>(&mut self) -> bool;

    fn play_patrol<const P: usize>(&mut self) -> bool;
    fn play_harem<const P: usize>(&mut self) -> bool;

    fn play_faithful_hound<const P: usize>(&mut self) -> bool;

    // supply inspection
    fn province_in_supply(&self) -> u8;
    fn colony_in_supply(&self) -> u8;
    fn colony_enabled(&self) -> bool;

    fn get_player<const P: usize>(&mut self) -> &mut PersonalState;
    fn get_observer(&mut self) -> &mut Self::Observer;

    // hooked effects
    fn add_coin<const P: usize>(&mut self, c: u32);
    fn draw_to<const P: usize>(&mut self) -> Option<CardType>;
    fn draw<const P: usize>(&mut self);

    // internal stuff
    fn end(&self) -> bool;
    fn clean_up<const P: usize>(&mut self);
}

pub struct Game<'a, K: kingdom::Kingdom, O: observer::Observer, RNG: rand::Rng + ?Sized, T: RoundPlayer + table::Table, const N: usize>
{
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

    faithful_hound: pile::faithful_hound::Pile,

    players: [PersonalState; N],
    trash: Vec<CardType>,

    observer: &'a mut O,
    rng: &'a mut RNG,
    table: T,
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
    pub vp: u32,
    pub coffer: u32,
    pub villager: u32,
    pub favor: u32,
}

impl PersonalState {
    pub fn make(use_shelter: bool) -> PersonalState {
        let mut ret = PersonalState {
            deck: vec![],
            discard: vec![
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
            vp: 0,
            coffer: 0,
            villager: 0,
            favor: 0,
        };
        ret.deck_stats[CardType::Copper as usize] = 7;
        if use_shelter {
            ret.discard.push(CardType::Hovel);
            ret.discard.push(CardType::Necropolis);
            ret.discard.push(CardType::OvergrownEstate);
            ret.deck_stats[CardType::Hovel as usize] += 1;
            ret.deck_stats[CardType::Necropolis as usize] += 1;
            ret.deck_stats[CardType::OvergrownEstate as usize] += 1;
        } else {
            ret.discard.push(CardType::Estate);
            ret.discard.push(CardType::Estate);
            ret.discard.push(CardType::Estate);
            ret.deck_stats[CardType::Estate as usize] = 3;
        }
        ret
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

    // only guarantees meaningful results at game end
    pub fn total_final_vp(&self) -> u32 {
        self.count_card_static::<{ CardType::Colony as usize }>() * 10
            + self.count_card_static::<{ CardType::Province as usize }>() * 6
            + self.count_card_static::<{ CardType::Duchy as usize }>() * 3
            + self.count_card_static::<{ CardType::Estate as usize }>() * 1
            + self.count_card_static::<{ CardType::Harem as usize }>() * 2
            + self.vp
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

impl<
        'a,
        K: kingdom::Kingdom + Default,
        O: observer::Observer,
        RNG: rand::Rng + ?Sized,
        T: RoundPlayer + table::Table,
        const N: usize,
    > Game<'a, K, O, RNG, T, N>
{
    pub fn make(o: &'a mut O, rng: &'a mut RNG, t: T) -> Game<'a, K, O, RNG, T, N> {
        let k = K::default();
        let ret: Game<'a, K, O, RNG, T, N> = Game {
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
            faithful_hound: pile::faithful_hound::Pile::make::<N>(),
            players: [(); N].map(|_| PersonalState::make(k.use_shelter())),
            trash: vec![],
            observer: o,
            rng: rng,
            table: t,
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

    pub fn run(&mut self) {
        for _card in 0..5 {
            self.draw::<0>();
            self.draw::<1>();
        }
        let mut break_pos: u32 = 0;
        for round in 0..100 {
            let ret = self.table.run_round::<Self>(self, round);
            if ret >= 0 {
                break_pos = ret as u32;
                break;
            }
        }
        let vp_0 = self.players[0].total_final_vp();
        let vp_1 = self.players[1].total_final_vp();
        let ret = if vp_0 > vp_1 {
            [0, 1]
        } else if vp_0 < vp_1 {
            [1, 0]
        } else if break_pos == 0 {
            [1, 0]
        } else {
            [0, 0]
        };
        self.observer.result_2(ret);
    }
}

impl<K: kingdom::Kingdom + Default, O: observer::Observer, RNG: rand::Rng + ?Sized, T: RoundPlayer + table::Table, const N: usize>
    GameState for Game<'_, K, O, RNG, T, N>
{
    type Observer = O;
    type Table = T;
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
    make_simple_buy_fn!(faithful_hound, buy_faithful_hound);

    make_simple_play_fn!(Gold, gold, play_gold);
    make_simple_play_fn!(Silver, silver, play_silver);
    make_simple_play_fn!(Copper, copper, play_copper);

    make_simple_play_fn!(Platinum, platinum, play_platinum);

    make_simple_play_fn!(Necropolis, necropolis, play_necropolis);

    make_simple_play_fn!(Smithy, smithy, play_smithy);
    make_simple_play_fn!(Village, village, play_village);

    make_simple_play_fn!(Harem, harem, play_harem);
    make_simple_play_fn!(Patrol, patrol, play_patrol);
    make_simple_play_fn!(FaithfulHound, faithful_hound, play_faithful_hound);

    #[inline]
    fn get_player<const P: usize>(&mut self) -> &mut PersonalState {
        &mut self.players[P]
    }

    #[inline]
    fn get_observer(&mut self) -> &mut O {
        self.observer
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
        if self.players[P].deck.len() == 0 {
            self.players[P].discard.shuffle(&mut self.rng);
            mem::swap(&mut self.players[P].deck, &mut self.players[P].discard);
        }
        self.players[P].deck.pop()
    }

    #[inline]
    fn draw<const P: usize>(&mut self) {
        let card = self.draw_to::<P>();
        match card {
            None => {}
            Some(x) => {
                self.players[P].hand[x as usize] += 1;
            }
        }
    }

    fn end(&self) -> bool {
        self.province_end() || self.colony_end() || self.pile_end()
    }

    fn clean_up<const P: usize>(&mut self) {
        for i in 0..CARDTYPES {
            for _j in 0..self.players[P].hand[i] {
                self.players[P].discard.push(FromPrimitive::from_usize(i).unwrap());
            }
            self.players[P].hand[i] = 0;
        }
        self.players[P].discard.append(&mut self.players[P].play);
        for _card in 0..5 {
            self.draw::<P>();
        }
    }
}
