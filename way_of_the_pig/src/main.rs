use way_of_the_pig::game;
use way_of_the_pig::kingdom;
use way_of_the_pig::controller::big_money;
use way_of_the_pig::controller::big_money_naive;

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
    let mut w1 : u32 = 0;
    let mut w2 : u32 = 0;
    for _i in 0..200000 {
        let mut a: game::Game<kingdom::SimpleKingdom, 2> = game::Game::make();
        let mut p1 : big_money::BigMoneyController = big_money::BigMoneyController::make();
        let mut p2 : big_money_naive::BigMoneyController = big_money_naive::BigMoneyController::make();
        //round_robin!(a; p1, p2);
        let result = a.run_random(&mut p1, &mut p2);
        if result == [0, 1] {
            w1 += 1;
        }
        if result == [1, 0] {
            w2 += 1;
        }
    }
    println!("p1 wins {}, p2 wins {}, draw {}", w1, w2, 200000 - w1 - w2);
}
