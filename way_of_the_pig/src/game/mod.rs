use crate::game::table::Table;
use crate::game_state;
use crate::kingdom;
use crate::observer;
use crate::pile;
use crate::pile::Pile;
use num_traits::FromPrimitive;
use rand::seq::SliceRandom;
use std::mem;

mod table;

pub trait Game {
    type GameState: game_state::GameState;
    type Observer: observer::Observer;

    fn get_game_state(&mut self) -> &mut Self::GameState;
    fn get_observer(&mut self) -> &mut Self::Observer;
}

pub struct GameImpl<'a, G: game_state::GameState, O: observer::Observer> {
    pub game: G,
    pub observer: &'a mut O,
}

impl<'a, G: game_state::GameState, O: observer::Observer> GameImpl<'a, G, O> {
    fn get_game_state(&mut self) -> &mut G {
        return &mut self.game;
    }
    fn get_observer(&mut self) -> &mut O {
        return self.observer
    }
}
