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

impl Default for RoundStats {
    fn default() -> Self {
        RoundStats {
            coin: [[0; MAX_ROUND]; 2],
            vp: [[0; MAX_ROUND]; 2],
            round: [[0; MAX_ROUND]; 2],
            current_round: 0,
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
                let ar = self.round[i][j];
                let br = other.round[1 - i][j];
                if ar == 0 && br == 0 {
                    break;
                }
                let ac = (a as f32) / (ar as f32);
                let bc = (b as f32) / (br as f32);
                let s2 = format!(
                    "P{} R{}: {:.3} ({:.3} {:.3})\n",
                    i,
                    j,
                    ((a + b) as f32) / ((ar + br) as f32),
                    ac,
                    bc
                );
                s.push_str(&s2);
            }
        }

        s
    }
}
