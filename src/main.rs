use dominion_simulator::game;
use dominion_simulator::kingdom;

fn main() {
    let mut a: game::Game<kingdom::SimpleKingdom> = game::Game::make(2);

    a.run();
}
