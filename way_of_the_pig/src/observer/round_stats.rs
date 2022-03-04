use crate::observer;

#[derive(Copy, Clone)]
pub struct RoundStats {
    coin: [u32; 100],
    vp: [u32; 100],
    round: [u32; 100],
}

impl observer::Observer for RoundStats {}
