use way_of_the_pig::game;
use way_of_the_pig::kingdom;
use way_of_the_pig::controller::big_money;

fn main() {
    let mut w1 : u32 = 0;
    for _i in 0..1000000 {
        let mut a: game::Game<kingdom::SimpleKingdom, 2> = game::Game::make();
        let mut p1 : big_money::BigMoneyController<kingdom::SimpleKingdom, 2> = big_money::BigMoneyController::make();
        let mut p2 : big_money::BigMoneyController<kingdom::SimpleKingdom, 2> = big_money::BigMoneyController::make();
        let result = a.run(&mut p1, &mut p2);
        if result == 0 {
            w1 += 1;
        }
    }
    println!("p1 wins {}, p2 wins {}", w1, 1000000 - w1);
}
