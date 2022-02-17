pub mod province;
pub mod smithy;

pub trait Card {
    fn static_price() -> u32;
}
