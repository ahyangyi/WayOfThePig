pub mod default;
pub mod round_stats;

pub trait Observer {
    fn start(&mut self) {}
    fn result_2(&mut self, _result: [u8; 2]) {}
    fn turn<const P: usize>(&mut self, _round: u32) {}
    fn add_coin<const P: usize>(&mut self, _c: u32) {}
}
