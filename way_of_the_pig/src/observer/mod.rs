pub mod default;
pub mod round_stats;

pub trait Observer {
    fn notify_result_2(&mut self, _result: [u8; 2]) {}
}
