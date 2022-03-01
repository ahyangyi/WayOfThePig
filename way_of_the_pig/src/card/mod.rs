use bitflags::bitflags;
use crate::game;
use num_derive::FromPrimitive;

pub mod province;
pub mod duchy;
pub mod estate;
pub mod gold;
pub mod silver;
pub mod copper;
pub mod curse;
pub mod platinum;
pub mod smithy;
pub mod patrol;

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
    }
}

pub trait Card {
    fn static_price() -> u32;
    fn static_type() -> Type;
    fn play<G: game::GameState, const P: usize>(_g: &mut G) {}
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

macro_rules! make_dynamic_dispatch_fn {
    ( $f:ident, $m:ident, $t:ty, $d:expr ) => {
        pub fn $f(c: CardType) -> $t {
            match c {
                CardType::Province => {province::Card::$m()},
                CardType::Duchy => {duchy::Card::$m()},
                CardType::Estate => {estate::Card::$m()},

                CardType::Platinum => {platinum::Card::$m()},

                _ => {$d}
            }
        }
    };
}

make_dynamic_dispatch_fn!(static_type, static_type, Type, Type::NONE);
