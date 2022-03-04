use crate::observer;

const MAX_ROUND: usize = 100;

#[derive(Copy, Clone)]
pub struct RoundStats {
    coin: [u32; MAX_ROUND],
    vp: [u32; MAX_ROUND],
    round: [u32; MAX_ROUND],
}

impl observer::Observer for RoundStats {
    fn notify_round(&mut self, round: u32) {
        if round < MAX_ROUND as u32 {
            self.round[round as usize] += 1;
        }
    }
}
