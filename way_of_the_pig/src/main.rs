use way_of_the_pig::game;
use way_of_the_pig::kingdom;
use way_of_the_pig::controller::big_money;
use way_of_the_pig::controller::smithy;

#[macro_export]
macro_rules! round_robin {
    ( @match $f:ident; $x:ident; $y:ident ) => {
        $f.run_random(&mut $x, &mut $y);
    };
    ( @match $f:ident; $x:ident; $y:ident, $($tail:ident),* ) => {
        $f.run_random(&mut $x, &mut $y);
        round_robin!(@match $f; $x; $($tail)*);
    };
    ( $f:ident ; $x:ident ) => {
    };
    ( $f:ident ; $x:ident, $($tail:ident),* ) => {
        round_robin!(@match $f; $x; $($tail)*);
        round_robin!($f; $($tail)*);
    };
}

fn main() {
    let mut w : u32 = 0;
    let mut p1 : big_money::BigMoneyController = big_money::BigMoneyController::make();
    let mut p2 : smithy::BigMoneyController = smithy::BigMoneyController::make();
    //round_robin!(a; p1, p2);
    for _i in 0..100000 {
        let mut a: game::Game<kingdom::SimpleKingdom, 2> = game::Game::make();
        let result = a.run(&mut p1, &mut p2);
        if result == [0, 1] {
            w += 2;
        } else if result == [0, 0] {
            w += 1;
        }
    }
    for _i in 0..100000 {
        let mut a: game::Game<kingdom::SimpleKingdom, 2> = game::Game::make();
        let result = a.run(&mut p2, &mut p1);
        if result == [1, 0] {
            w += 2;
        } else if result == [0, 0] {
            w += 1;
        }
    }
    println!("{} - {}", w, 400000 - w);
}
