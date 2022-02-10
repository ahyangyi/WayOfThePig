use dominion_simulator::game;
use dominion_simulator::kingdom;

fn main() {
    let a: game::Game<kingdom::SimpleKingdom> = game::Game::make();
    println!("Hello, world!");
}
