use crate::controller::Controller;

pub trait Table {
    type Controller0: Controller;
    type Controller1: Controller;

    fn c0(&mut self) -> &mut Self::Controller0;
    fn c1(&mut self) -> &mut Self::Controller1;
}

impl<T0: Controller, T1: Controller> Table for (&mut T0, &mut T1) {
    type Controller0 = T0;
    type Controller1 = T1;

    fn c0(&mut self) -> &mut Self::Controller0 {
        &mut self.0
    }
    fn c1(&mut self) -> &mut Self::Controller1 {
        &mut self.1
    }
}
