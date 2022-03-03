use way_of_the_pig::controller::big_money;
use way_of_the_pig::controller::patrol;
use way_of_the_pig::controller::patrol_harem;
use way_of_the_pig::controller::smithy;
use way_of_the_pig::controller::smithy_accidental_village;
use way_of_the_pig::game;
use way_of_the_pig::kingdom;
use way_of_the_pig::observer;

#[macro_export]
macro_rules! round_robin {
    ( @match $f:ident; $w:ident; $n:expr; $i:expr; $j:expr; $x:ident; $y:ident ) => {
        for _i in 0..$n {
            let mut a: game::Game<kingdom::SimpleKingdom, observer::default::WinDrawLoss, 2> = game::Game::make(&mut $w[$i][$j]);
            a.run(&mut $x, &mut $y);
        }
        for _i in 0..$n {
            let mut a: game::Game<kingdom::SimpleKingdom, observer::default::WinDrawLoss, 2> = game::Game::make(&mut $w[$j][$i]);
            a.run(&mut $y, &mut $x);
        }
    };
    ( @match $f:ident; $w:ident; $n:expr; $i:expr; $j:expr; $x:ident; $y:ident, $($tail:ident),* ) => {
        round_robin!(@match $f; $w; $n; $i; $j; $x; $y);
        round_robin!(@match $f; $w; $n; $i; $j+1usize; $x; $($tail),*);
    };
    ( @match_array $f:ident; $w:ident; $n:expr; $i:expr; $x:ident ) => {
    };
    ( @match_array $f:ident; $w:ident; $n:expr; $i:expr; $x:ident, $($tail:ident),* ) => {
        round_robin!(@match $f; $w; $n; $i; $i+1usize; $x; $($tail),*);
        round_robin!(@match_array $f; $w; $n; $i+1usize; $($tail),*);
    };
    ( $f:ident; $w:ident; $n:expr; $($tail:ident),* ) => {
        round_robin!(@match_array $f; $w; $n; 0usize; $($tail),*);
    };
}

fn main() {
    let mut w = [[observer::default::WinDrawLoss::default(); 5]; 5];
    let mut p1: big_money::Controller = big_money::Controller::make();
    let mut p2: smithy::Controller = smithy::Controller::make();
    // let mut p2a: smithy_accidental_village::Controller = smithy_accidental_village::Controller::make();
    let mut p3: patrol::Controller = patrol::Controller::make();
    let mut p4: patrol_harem::Controller = patrol_harem::Controller::make();
    let n = 100000;
    // round_robin!(a; w; p1, p2, p2a, p3, p4);
    round_robin!(a; w; n; p1, p2, p3, p4);

    // let names = ["bm", "smithy", "smithy_accidental_village", "patrol", "patrol+harem"];
    let names = ["bm", "smithy", "patrol", "patrol+harem"];
    //for i in 0..5 {
    //    for j in i + 1..5 {
    for i in 0..4 {
        for j in i + 1..4 {
            let p1 = (w[i][j].win * 2 + w[i][j].draw) as f64 / (2.0 * n as f64);
            let p2 = (w[j][i].loss * 2 + w[j][i].draw) as f64 / (2.0 * n as f64);
            println!("{} vs {}: {:.3} ({:.3}; {:.3})", names[i], names[j], (p1 + p2) / 2.0, p1, p2);
        }
    }
}
