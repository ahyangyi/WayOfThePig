use crate::observer;

#[derive(Copy, Clone)]
pub struct WinDrawLoss {
    pub win: u32,
    pub draw: u32,
    pub loss: u32,
}

impl WinDrawLoss {
    pub fn default() -> WinDrawLoss {
        WinDrawLoss { win: 0, draw: 0, loss: 0 }
    }
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
