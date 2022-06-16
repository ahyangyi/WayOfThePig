use crate::card::CardType;
use crate::game_state;

pub mod big_money;
pub mod faithful_hound;
pub mod patrol;
pub mod patrol_harem;
pub mod smithy;
pub mod smithy_accidental_village;

pub trait Controller {
    fn act<G: game_state::GameState, const P: usize>(&mut self, _game: &mut G) {}
    fn buy<G: game_state::GameState, const P: usize>(&mut self, _game: &mut G) {}
    // FIXME None isn't even legal, but we don't have a sensible default currently
    fn trash_for_upgrade<G: game_state::GameState, const P: usize>(&mut self, _game: &mut G) -> Option<CardType> {
        None
    }
    fn discard_1<G: game_state::GameState, const P: usize>(&mut self, _game: &mut G) -> Option<CardType> {
        None
    }
}
