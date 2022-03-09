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

impl RoundStats {
    pub fn pair_stats(&self, other: &Self) -> String {
        let mut s: String = Default::default();

        for i in 0..2 {
            for j in 0..MAX_ROUND {
                let a = self.coin[i][j];
                let b = other.coin[1 - i][j];
                let s2 = format!("P{} R{}: {:.3} ({:.3}; {:.3})\n", i, j, ((a + b) as f32) / 2.0, a, b);
                s.push_str(&s2);
            }
        }

        s
    }
}
