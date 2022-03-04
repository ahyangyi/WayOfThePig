use crate::observer;

const MAX_ROUND: usize = 100;

#[derive(Copy, Clone)]
pub struct RoundStats {
    coin: [[u32; MAX_ROUND]; 2],
    vp: [[u32; MAX_ROUND]; 2],
    round: [[u32; MAX_ROUND]; 2],
}

impl observer::Observer for RoundStats {
    fn notify_round(&mut self, round: u32) {
        if round < MAX_ROUND as u32 {
            self.round[0][round as usize] += 1;
        }
    }
}
