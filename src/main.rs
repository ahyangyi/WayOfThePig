use dominion_simulator::game;
use dominion_simulator::kingdom;
use rand::Rng;

fn main() {
    let a: game::Game<kingdom::SimpleKingdom> = game::Game::make(2);

    let mut x: u32 = 0;
    for i in 0..100000000 {
        x += rand::thread_rng().gen_range(1..101);
    }
    println!("{}", x);
}
