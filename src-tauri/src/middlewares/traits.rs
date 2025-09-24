pub trait Middleware<I, O> {
    fn handle(&self, input: I, next: impl FnOnce(I) -> O) -> O;
}
