pub mod default;

pub trait Observer {
    fn notify_result_2(&mut self, result: [u8; 2]);
}
