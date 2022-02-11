trait Controller {
    fn make() -> Controller;
    fn act(&self);
    fn buy(&self);
}
