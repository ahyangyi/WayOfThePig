use crate::game;
use bitflags::bitflags;
use num_derive::FromPrimitive;

// Base
pub mod copper;
pub mod curse;
pub mod duchy;
pub mod estate;
pub mod gold;
pub mod province;
pub mod silver;

// Colony
pub mod colony;
pub mod platinum;

// Shelter
pub mod hovel;
pub mod necropolis;
pub mod overgrown_estate;

// Base Set
pub mod festival;
pub mod market;
pub mod militia;
pub mod smithy;
pub mod village;

// Intrigue
pub mod harem;
pub mod patrol;
pub mod upgrade;

// Hinterlands
pub mod oasis;

// Nocturne
pub mod faithful_hound;

bitflags! {
    pub struct Type: u32 {
        const NONE = 0;

        // Basic types
        const ACTION = 0x1;
        const TREASURE = 0x2;
        const VICTORY = 0x4;
        const CURSE = 0x8;

        // Multi-expansion types
        const ATTACK = 0x10;
        const DURATION = 0x20;
        const REACTION = 0x40;
        const COMMAND = 0x80;

        // Single-expansion types;
        const DOOM = 0x100;
        const FATE = 0x200;
        const HEIRLOOM = 0x400;
        const LIAISON = 0x800;
        const LOOTER = 0x1000;
        const NIGHT = 0x2000;
        const RESERVE = 0x4000;
        const RUINS = 0x8000;
        const SHELTER = 0x10000;

        // Split piles
        const KNIGHT = 0x100000;
        const CASTLE = 0x200000;
        const AUGUR = 0x400000;
        const FORT = 0x800000;
        const ODYSSEY = 0x1000000;
        const TOWNSFOLK = 0x2000000;
        const WIZARD = 0x4000000;
    }
}

pub trait Card {
    fn static_price() -> u32;
    fn static_type() -> Type;
    fn on_play<G: game::GameState, const P: usize>(_g: &mut G) {}
    #[inline]
    fn vp() -> u32 {
        0
    }
}

#[derive(Copy, Clone, PartialEq, Debug, FromPrimitive)]
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

macro_rules! make_dynamic_dispatch_fn {
    ( $f:ident, $m:ident, $t:ty, $d:expr ) => {
        #[inline]
        pub fn $f(c: CardType) -> $t {
            match c {
                CardType::Province => province::Card::$m(),
                CardType::Duchy => duchy::Card::$m(),
                CardType::Estate => estate::Card::$m(),
                CardType::Gold => gold::Card::$m(),
                CardType::Silver => silver::Card::$m(),
                CardType::Copper => copper::Card::$m(),

                CardType::Colony => colony::Card::$m(),
                CardType::Platinum => platinum::Card::$m(),

                CardType::Hovel => hovel::Card::$m(),
                CardType::Necropolis => necropolis::Card::$m(),
                CardType::OvergrownEstate => overgrown_estate::Card::$m(),

                CardType::Market => market::Card::$m(),
                CardType::Militia => militia::Card::$m(),
                CardType::Smithy => smithy::Card::$m(),
                CardType::Village => village::Card::$m(),

                CardType::Harem => harem::Card::$m(),
                CardType::Patrol => patrol::Card::$m(),
                CardType::Upgrade => upgrade::Card::$m(),

                CardType::Oasis => oasis::Card::$m(),

                CardType::FaithfulHound => faithful_hound::Card::$m(),

                _ => $d,
            }
        }
    };
}

make_dynamic_dispatch_fn!(static_type, static_type, Type, Type::NONE);
make_dynamic_dispatch_fn!(static_price, static_price, u32, 0);
