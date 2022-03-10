use crate::observer;

#[derive(Copy, Clone, Default)]
pub struct WinDrawLoss {
    pub win: u32,
    pub draw: u32,
    pub loss: u32,
}

impl observer::Observer for WinDrawLoss {
    #[inline]
    fn notify_result_2(&mut self, result: [u8; 2]) {
        if result[0] == 0 && result[1] == 1 {
            self.win += 1;
        } else if result[0] == 0 && result[1] == 0 {
            self.draw += 1;
        } else {
            self.loss += 1;
        }
    }
}

impl WinDrawLoss {
    #[inline]
    pub fn pair_stats(&self, other: &Self) -> String {
        let n = self.win + self.draw + self.loss;
        let p1 = (self.win * 2 + self.draw) as f64 / (2.0 * n as f64);
        let p2 = (other.loss * 2 + other.draw) as f64 / (2.0 * n as f64);
        format!("{:.3} ({:.3}; {:.3})", (p1 + p2) / 2.0, p1, p2)
    }
}
