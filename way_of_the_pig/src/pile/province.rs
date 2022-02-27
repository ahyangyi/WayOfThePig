use crate::pile;

pub struct Pile {
    province: u8,
}

impl pile::Pile for Pile {
    #[inline]
    fn make() -> Self {
        // FIXME depend on player count
        Pile {
            province: 8,
        }
    }

    #[inline]
    fn enabled(&self) -> bool {
        true
    }

    #[inline]
    fn pop(&mut self) {
        if self.province > 0 {
            self.province -= 1
        }
    }

    #[inline]
    fn remaining_cards(&self) -> u8 {
        self.province
    }
}
