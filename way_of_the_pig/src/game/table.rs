use crate::controller::Controller;
use crate::game::Game;
use crate::observer::Observer;

pub trait Table {
    type Controller0: Controller;
    type Controller1: Controller;

    fn c0(&mut self) -> &mut Self::Controller0;
    fn c1(&mut self) -> &mut Self::Controller1;
    fn run_round<G>(&mut self, game: &mut G, round: u32) -> i8
    where
        G: Game;
}

impl<T0: Controller, T1: Controller> Table for (&mut T0, &mut T1) {
    type Controller0 = T0;
    type Controller1 = T1;

    fn c0(&mut self) -> &mut Self::Controller0 {
        &mut self.0
    }
    fn c1(&mut self) -> &mut Self::Controller1 {
        &mut self.1
    }

    #[inline]
    fn run_round<G>(&mut self, game: &mut G, round: u32) -> i8
    where
        G: Game,
    {
        game.get_observer().turn::<0>(round);
        game.get_game_state().get_player::<0>().turn_start();
        self.0.act::<G, 0>(game.get_game_state());
        self.0.buy::<G, 0>(game.get_game_state());
        if game.get_game_state().end() {
            return 0;
        }
        game.clean_up::<0>();
        game.get_observer().turn::<1>(round);
        game.get_game_state().get_player::<1>().turn_start();
        self.1.act::<G, 1>(game.get_game_state());
        self.1.buy::<G, 1>(game.get_game_state());
        if game.get_game_state().end() {
            return 1;
        }
        game.get_game_state().clean_up::<1>();
        -1
    }
}
