pub trait With: Sized {
    fn with<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut Self),
    {
        f(&mut self);
        self
    }

    fn try_with<F, E>(mut self, f: F) -> Result<Self, E>
    where
        F: FnOnce(&mut Self) -> Result<(), E>,
    {
        match f(&mut self) {
            Ok(_) => Ok(self),
            Err(e) => Err(e),
        }
    }
}

impl<T: Sized> With for T {}
