use crate::observer;

const MAX_ROUND: usize = 100;

#[derive(Copy, Clone)]
pub struct RoundStats {
    coin: [[u32; MAX_ROUND]; 2],
    vp: [[u32; MAX_ROUND]; 2],
    round: [[u32; MAX_ROUND]; 2],
    current_round: u32,
}

impl observer::Observer for RoundStats {
    fn notify_turn<const P: usize>(&mut self, round: u32) {
        self.current_round = round;
        if round < MAX_ROUND as u32 {
            self.round[P][round as usize] += 1;
        }
    }
    fn add_coin<const P: usize>(&mut self, c: u32) {
        if self.current_round < MAX_ROUND as u32 {
            self.coin[P][self.current_round as usize] += c;
        }
    }
}
