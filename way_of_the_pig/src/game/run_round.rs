use crate::controller;
use crate::game::GameState;
use crate::observer::Observer;

pub trait RoundPlayer {
    fn run_round<G>(&mut self, game: &mut G, round: u32) -> i8
    where
        G: GameState;
}

impl<T1, T2> RoundPlayer for (&mut T1, &mut T2)
where
    T1: controller::Controller,
    T2: controller::Controller,
{
    #[inline]
    fn run_round<G>(&mut self, game: &mut G, round: u32) -> i8
    where
        G: GameState,
    {
        game.get_observer().notify_turn::<0>(round);
        game.get_player::<0>().turn_start();
        self.0.act::<G, 0>(game);
        self.0.buy::<G, 0>(game);
        if game.end() {
            return 0;
        }
        game.get_player::<0>().clean_up();
        game.get_observer().notify_turn::<1>(round);
        game.get_player::<1>().turn_start();
        self.1.act::<G, 1>(game);
        self.1.buy::<G, 1>(game);
        if game.end() {
            return 1;
        }
        game.get_player::<1>().clean_up();
        -1
    }
}
