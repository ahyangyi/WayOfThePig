use crate::game::table::Table;
use crate::game_state;
use crate::kingdom;
use crate::observer;
use crate::pile;
use crate::pile::Pile;
use num_traits::FromPrimitive;
use rand::seq::SliceRandom;
use std::mem;

mod table;

pub trait Game {
    type GameState: game_state::GameState;
    type Observer: observer::Observer;

    fn get_game_state(&mut self) -> &mut GameState;
    fn get_observer(&mut self) -> &mut Observer;
}

pub struct<'a, G: game_state::GameState> GameImpl {
    pub game: G;
}

impl<'a, K: kingdom::Kingdom + Default, O: observer::Observer, RNG: rand::Rng + ?Sized, const N: usize> Game<'a, K, O, RNG, N> {
    pub fn make(o: &'a mut O, rng: &'a mut RNG) -> Game<'a, K, O, RNG, N> {
        let k = K::default();
        let ret: Game<'a, K, O, RNG, N> = Game {
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
            rng,
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

    pub fn run<T1, T2>(&mut self, t1: &mut T1, t2: &mut T2)
    where
        T1: controller::Controller,
        T2: controller::Controller,
    {
        for _card in 0..5 {
            self.draw::<0>();
            self.draw::<1>();
        }
        let mut break_pos: u32 = 0;
        for round in 0..100 {
            let ret = (&mut *t1, &mut *t2).run_round::<Self>(self, round);
            if ret >= 0 {
                break_pos = ret as u32;
                break;
            }
        }
        let vp_0 = self.players[0].total_final_vp();
        let vp_1 = self.players[1].total_final_vp();
        let ret = if vp_0 > vp_1 {
            [0, 1]
        } else if vp_0 < vp_1 || break_pos == 0 {
            [1, 0]
        } else {
            [0, 0]
        };
        self.observer.result_2(ret);
    }
}

impl<K: kingdom::Kingdom + Default, O: observer::Observer, RNG: rand::Rng + ?Sized, const N: usize> GameState
    for Game<'_, K, O, RNG, N>
{
    type Observer = O;
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

    make_simple_play_action_fn!(Necropolis, necropolis, play_necropolis);

    make_simple_play_action_fn!(Smithy, smithy, play_smithy);
    make_simple_play_action_fn!(Village, village, play_village);

    make_simple_play_fn!(Harem, harem, play_harem);
    make_simple_play_action_fn!(Patrol, patrol, play_patrol);
    make_simple_play_action_fn!(FaithfulHound, faithful_hound, play_faithful_hound);

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
        if self.players[P].deck.is_empty() {
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
