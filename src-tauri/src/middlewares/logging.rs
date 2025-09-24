use super::traits::Middleware;

pub struct Logger;

impl<I: std::fmt::Debug, O> Middleware<I, O> for Logger {
    fn handle(&self, input: I, next: impl FnOnce(I) -> O) -> O {
        println!("Before handling: {:?}", input);
        let output = next(input);
        println!("After handling");
        output
    }
}
