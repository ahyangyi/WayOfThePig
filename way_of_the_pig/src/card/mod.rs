use bitflags::bitflags;
use crate::game;
pub mod province;
pub mod duchy;
pub mod estate;
pub mod gold;
pub mod silver;
pub mod copper;
pub mod curse;
pub mod smithy;

bitflags! {
    pub struct Type: u32 {
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
    fn play<G: game::GameState, const P: usize>(g: &mut G) {}
}
