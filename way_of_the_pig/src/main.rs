use way_of_the_pig::game;
use way_of_the_pig::kingdom;
use way_of_the_pig::controller::big_money;
use way_of_the_pig::controller::smithy;
use way_of_the_pig::controller::patrol;

macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {$sub};
}

#[macro_export]
macro_rules! round_robin {
    ( @match $f:ident; $w:ident; $($i:ident),*; $x:ident; $y:ident ) => {
        for _i in 0..100000 {
            let mut a: game::Game<kingdom::SimpleKingdom, 2> = game::Game::make();
            let result = a.run(&mut $x, &mut $y);
            if result == [0, 1] {
                $w[0][0] += 2;
            } else if result == [0, 0] {
                $w[0][0] += 1;
            }
        }
        for _i in 0..100000 {
            let mut a: game::Game<kingdom::SimpleKingdom, 2> = game::Game::make();
            let result = a.run(&mut $y, &mut $x);
            if result == [1, 0] {
                $w[0][0] += 2;
            } else if result == [0, 0] {
                $w[0][0] += 1;
            }
        }
    };
    ( @match $f:ident; $w:ident; $($i:ident),*; $x:ident; $y:ident, $($tail:ident),* ) => {
        round_robin!(@match $f; $w; $($i),*; $x; $y);
        round_robin!(@match $f; $w; $($i),*; $x; $($tail),*);
    };
    ( @match_array $f:ident; $w:ident; $($i:ident),*; $x:ident ) => {
    };
    ( @match_array $f:ident; $w:ident; $($i:ident),*; $x:ident, $($tail:ident),* ) => {
        round_robin!(@match $f; $w; $($i),*; $x; $($tail),*);
        round_robin!(@match_array $f; $w; $($i),* ; $($tail),*);
    };
    ( $f:ident; $w:ident; $($tail:ident),* ) => {
        round_robin!(@match_array $f; $w; ; $($tail),*);
    };
}

fn main() {
    let mut w = [[0u32; 3]; 3];
    let mut p1 : big_money::BigMoneyController = big_money::BigMoneyController::make();
    let mut p2 : smithy::BigMoneyController = smithy::BigMoneyController::make();
    let mut p3 : patrol::BigMoneyController = patrol::BigMoneyController::make();
    {
        let mut a: game::Game<kingdom::SimpleKingdom, 2> = game::Game::make();
        round_robin!(a; w; p1, p2, p3);
    }
    for i in 0..3 {
        for j in 0..3 {
            println!("{} vs {}: {} - {}", i, j, w[i][j], 400000 - w[i][j]);
        }
    }
}
