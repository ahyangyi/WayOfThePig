struct Match<G: Game, C1: Controller, C2: Controller> {
    game: G,
    controller1: C1,
    controller2: C2,
}

impl Match {
    fn make() {
        Match {
            game: Game.make(),
            controller1: Controller1.make(),
            controller2: Controller2.make(),
        }
    }
    fn run(&self) {}
}
