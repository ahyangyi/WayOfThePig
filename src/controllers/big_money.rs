// An implementation of WanderingWinder's "Big Money Ultimate", which attempts to buy the first
// possible entry:
//   Buy province if total money > 18
//   Buy duchy if remaining province <= 4
//   Buy estate if remaining province <= 2
//   Buy gold
//   Buy Duchy if remaining province <= 6
//   Buy silver
trait Controller {
    fn buy(&mut self, game: mut &Game, hand: mut &PersonalState) {
        game.buy_gold() || game.buy_silver();
    }
}
