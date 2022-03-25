use way_of_the_pig::controller::big_money;
use way_of_the_pig::controller::faithful_hound;
use way_of_the_pig::controller::patrol;
use way_of_the_pig::controller::patrol_harem;
use way_of_the_pig::controller::smithy;
use way_of_the_pig::game;
use way_of_the_pig::kingdom;
use way_of_the_pig::observer;

#[macro_export]
macro_rules! round_robin {
    ( @match $f:ident; $w:ident; $n:expr; $i:expr; $j:expr; $tx:ident, $x:ident; $ty:ident, $y:ident ) => {
        let mut rng = rand::thread_rng();
        for _i in 0..$n {
            let mut a: game::Game<kingdom::SimpleKingdom, observer::default::WinDrawLoss, rand::rngs::ThreadRng, ($tx::Controller, $ty::Controller), 2> =
                game::Game::make(&mut $w[$i][$j], &mut rng, (&mut $x, &mut $y));
            a.run(&mut $x, &mut $y);
        }
        for _i in 0..$n {
            let mut a: game::Game<kingdom::SimpleKingdom, observer::default::WinDrawLoss, rand::rngs::ThreadRng, ($ty::Controller, $tx::Controller), 2> =
                game::Game::make(&mut $w[$j][$i], &mut rng, (&mut $y, &mut $x));
            a.run(&mut $y, &mut $x);
        }
    };
    ( @match $f:ident; $w:ident; $n:expr; $i:expr; $j:expr; $tx:ident, $x:ident; $ty:ident, $y:ident, $($tail:ident),* ) => {
        round_robin!(@match $f; $w; $n; $i; $j; $tx, $x; $ty, $y);
        round_robin!(@match $f; $w; $n; $i; $j+1usize; $tx, $x; $($tail),*);
    };
    ( @match_array $f:ident; $w:ident; $n:expr; $i:expr; $tx:ident, $x:ident ) => {
    };
    ( @match_array $f:ident; $w:ident; $n:expr; $i:expr; $tx:ident, $x:ident, $($tail:ident),* ) => {
        round_robin!(@match $f; $w; $n; $i; $i+1usize; $tx, $x; $($tail),*);
        round_robin!(@match_array $f; $w; $n; $i+1usize; $($tail),*);
    };
    // Entrance
    ( $f:ident; $w:ident; $n:expr; $($tail:ident),* ) => {
        round_robin!(@match_array $f; $w; $n; 0usize; $($tail),*);
    };
}

fn main() {
    let mut w = [[observer::default::WinDrawLoss::default(); 5]; 5];
    let mut p1: big_money::Controller = big_money::Controller::make();
    let mut p2: smithy::Controller = smithy::Controller::make();
    let mut p3: patrol::Controller = patrol::Controller::make();
    let mut p4: patrol_harem::Controller = patrol_harem::Controller::make();
    let mut p5: faithful_hound::Controller = faithful_hound::Controller::make();
    let n = 100000;
    round_robin!(a; w; n;
    big_money, p1,
    smithy, p2,
    patrol, p3,
    patrol_harem, p4,
    faithful_hound, p5
    );

    let names = ["bm", "smithy", "patrol", "patrol+harem", "faithful hound"];
    for i in 0..5 {
        for j in i + 1..5 {
            println!("{} vs {}: {}", names[i], names[j], w[i][j].pair_stats(&w[j][i]));
        }
    }
}
