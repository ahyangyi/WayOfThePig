use crate::pile;

pub trait Kingdom {
    type ColonyPile: pile::Pile;
    type PlatinumPile: pile::Pile;
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
