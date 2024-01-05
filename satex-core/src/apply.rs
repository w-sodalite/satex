pub trait Apply: Sized {
    fn apply<R, F: FnOnce(&mut Self) -> R>(self, f: F) -> Self;
}

impl<T> Apply for T {
    fn apply<R, F: FnOnce(&mut Self) -> R>(mut self, f: F) -> Self {
        f(&mut self);
        self
    }
}
