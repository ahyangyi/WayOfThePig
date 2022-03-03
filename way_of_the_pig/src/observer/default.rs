use crate::observer;

struct WinDrawLoss {
    win: u32,
    draw: u32,
    loss: u32,
}

impl observer::Observer for WinDrawLoss {
    fn notify_result(&mut self, result: &Vec<u8>) {
        debug_assert!(result.len() == 2);

        if result[0] == 0 && result[1] == 1 {
            self.win += 1;
        } else if result[0] == 0 && result[1] == 0 {
            self.draw += 1;
        } else {
            self.loss += 1;
        }
    }
}
