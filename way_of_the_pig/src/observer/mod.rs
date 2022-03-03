pub mod default;

pub trait Observer {
    fn notify_result(&mut self, result: &Vec<u8>);
}
