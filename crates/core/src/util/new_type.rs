#[macro_export]
macro_rules! new_type {
    ($(#[$attr:meta])* $name:ident, $ty:ty) => {

        $(#[$attr])*
        pub struct $name($ty);

        impl $name {
             pub fn new<I: Into<$ty>>(i: I) -> Self {
                Self(i.into())
            }

            pub fn take(self) -> $ty {
                self.0
            }
        }
        
        impl std::ops::Deref for $name {
            type Target = $ty;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}
