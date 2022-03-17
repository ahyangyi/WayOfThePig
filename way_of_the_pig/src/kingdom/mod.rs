use crate::pile;

pub trait Kingdom {
    // Colony
    type ColonyPile: pile::Pile;
    type PlatinumPile: pile::Pile;

    // Shelter
    fn use_shelter(&self) -> bool {
        false
    }
}

pub struct SimpleKingdom {}

impl Kingdom for SimpleKingdom {
    type ColonyPile = pile::none::Pile;
    type PlatinumPile = pile::none::Pile;
}

pub struct ProsperityKingdom {}

impl Kingdom for ProsperityKingdom {
    type ColonyPile = pile::colony::Pile;
    type PlatinumPile = pile::platinum::Pile;
}

pub struct DarkAgeKingdom {}

impl Kingdom for DarkAgeKingdom {
    type ColonyPile = pile::none::Pile;
    type PlatinumPile = pile::none::Pile;

    fn use_shelter(&self) -> bool {
        true
    }
}
