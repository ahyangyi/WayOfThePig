use way_of_the_pig::game;
use way_of_the_pig::kingdom;
use way_of_the_pig::controller::big_money;
use way_of_the_pig::controller::smithy;
use way_of_the_pig::controller::patrol;

#[macro_export]
macro_rules! round_robin {
    ( @match $f:ident; $w:ident; $i:expr; $j:expr; $x:ident; $y:ident ) => {
        for _i in 0..100000 {
            let mut a: game::Game<kingdom::SimpleKingdom, 2> = game::Game::make();
            let result = a.run(&mut $x, &mut $y);
            if result == [0, 1] {
                $w[$i][$j] += 2;
            } else if result == [0, 0] {
                $w[$i][$j] += 1;
            }
        }
        for _i in 0..100000 {
            let mut a: game::Game<kingdom::SimpleKingdom, 2> = game::Game::make();
            let result = a.run(&mut $y, &mut $x);
            if result == [0, 1] {
                $w[$j][$i] += 2;
            } else if result == [0, 0] {
                $w[$j][$i] += 1;
            }
        }
    };
    ( @match $f:ident; $w:ident; $i:expr; $j:expr; $x:ident; $y:ident, $($tail:ident),* ) => {
        round_robin!(@match $f; $w; $i; $j; $x; $y);
        round_robin!(@match $f; $w; $i; $j+1usize; $x; $($tail),*);
    };
    ( @match_array $f:ident; $w:ident; $i:expr; $x:ident ) => {
    };
    ( @match_array $f:ident; $w:ident; $i:expr; $x:ident, $($tail:ident),* ) => {
        round_robin!(@match $f; $w; $i; $i+1usize; $x; $($tail),*);
        round_robin!(@match_array $f; $w; $i+1usize; $($tail),*);
    };
    ( $f:ident; $w:ident; $($tail:ident),* ) => {
        round_robin!(@match_array $f; $w; 0usize; $($tail),*);
    };
}

fn main() {
    let mut w = [[0u32; 3]; 3];
    let mut p1 : big_money::Controller = big_money::Controller::make();
    let mut p2 : smithy::Controller = smithy::Controller::make();
    let mut p3 : patrol::Controller = patrol::Controller::make();
    round_robin!(a; w; p1, p2, p3);

    let names = ["bm", "smithy", "patrol"];
    for i in 0..3 {
        for j in i+1..3 {
            let p1 = w[i][j] as f64 / 200000.0;
            let p2 = 1.0 - (w[j][i] as f64 / 200000.0);
            println!("{} vs {}: {:.3} ({:.3}; {:.3})", names[i], names[j], (p1+p2)/2.0, p1, p2);
        }
    }
}
