use way_of_the_pig::game;
use way_of_the_pig::kingdom;
use way_of_the_pig::controller;

fn main() {
    let mut p1 : u32 = 0;
    for _i in 0..1000000 {
        let mut a: game::Game<kingdom::SimpleKingdom, 2> = game::Game::make();
        let result = a.run();
        if result == 0 {
            p1 += 1;
        }
    }
    println!("p1 wins {}, p2 wins {}", p1, 1000000 - p1);
}
