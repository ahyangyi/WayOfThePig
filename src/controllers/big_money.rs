trait Controller {
    fn buy(&mut self, game: &Game) {
        game.buy_gold() || game.buy_silver();
    }
}
