use crate::controller::Controller;

pub trait Table {
    type Controller0: Controller;
    type Controller1: Controller;

    fn get_controller0<const P: usize>(&mut self) -> &mut Self::Controller0;
    fn get_controller1<const P: usize>(&mut self) -> &mut Self::Controller1;
}

impl<T0: Controller, T1: Controller> Table for (T0, T1) {
    type Controller0 = T0;
    type Controller1 = T1;

    fn get_controller0<const P: usize>(&mut self) -> &mut Self::Controller0 {
        &mut self.0
    }
    fn get_controller1<const P: usize>(&mut self) -> &mut Self::Controller1 {
        &mut self.1
    }
}
