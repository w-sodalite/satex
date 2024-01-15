#[macro_export]
macro_rules! export_make {
    ($name:ident) => {
        $crate::concat_idents!(make_registry=$name, Registry {
            pub use registry::make_registry;
        });
        $crate::concat_idents!(arc_make=Arc, $name {
            pub use make::{arc_make, $name};
        });
    };
}

#[macro_export]
macro_rules! make {
    (@compcat,$make_name:ident, $make_out_name:ident, ($($make_out_type:tt)+), $arc_make_out_type:ty) =>{
        make!(@compcat, $make_name, $make_out_name, ($($make_out_type)+), ($crate::config::args::Args), $arc_make_out_type);
    };

    (@compcat,$make_name:ident, $make_out_name:ident, ($($make_out_type:tt)+), ($($args_type:tt)+), $arc_make_out_type:ty) => {

        pub trait $make_name {
            type $make_out_name: $($make_out_type)+;

            fn name(&self) -> &'static str;

            fn make(&self, args: $($args_type)+) -> Result<Self::$make_out_name, $crate::Error>;
        }

        struct MakeFn<M, F> {
            make: M,
            f: F,
        }

        impl<M, F> MakeFn<M, F> {
            pub fn new(make: M, f: F) -> Self {
                Self { make, f }
            }
        }

        impl<M, F, T> $make_name for MakeFn<M, F>
        where
            M: $make_name,
            F: Fn(M::$make_out_name) -> T,
            T: $($make_out_type)+,
        {
            type $make_out_name = T;

            fn name(&self) -> &'static str {
                self.make.name()
            }

            fn make(&self, args: $($args_type)+) -> Result<Self::$make_out_name,$crate::Error> {
                self.make.make(args).map(|out| (self.f)(out))
            }
        }

        $crate::concat_idents!(arc_make = Arc, $make_name {

            #[derive(Clone)]
            pub struct arc_make(
                std::sync::Arc<
                    dyn $make_name<$make_out_name = $arc_make_out_type> + Send + Sync + 'static,
                >,
            );

            impl $make_name for arc_make {
                type $make_out_name = $arc_make_out_type;

                fn name(&self) -> &'static str {
                    self.0.name()
                }

                fn make(&self, args: $($args_type)+) -> Result<Self::$make_out_name, $crate::Error> {
                    self.0.make(args)
                }
            }
        });
    };

    ($make_name:ident, $make_out_name:ident, ($($make_out_type:tt)+), $arc_make_out_type:ty) => {
        make!($make_name, $make_out_name, ($($make_out_type)+), ($crate::config::args::Args), $arc_make_out_type);
    };

    ($make_name:ident, $make_out_name:ident, ($($make_out_type:tt)+), ($($args_type:tt)+), $arc_make_out_type:ty) => {

        make!(@compcat, $make_name, $make_out_name, ($($make_out_type)+), ($($args_type)+), $arc_make_out_type);

        $crate::concat_idents!(arc_make = Arc, $make_name {
            impl arc_make {
                pub fn new<M, O>(make: M) -> Self
                    where
                        M: $make_name<$make_out_name = O> + Send + Sync + 'static,
                        O: $($make_out_type)+ + Send + Sync + 'static,
                    {
                        let name = make.name();
                        Self(std::sync::Arc::new(MakeFn::new(
                            make,
                            |inner| <$arc_make_out_type>::new(name,inner),
                        )))
                    }
            }
        });
    };
}

#[macro_export]
macro_rules! make_impl {
    (@internal $make_trait:ident,$classify:ident,$name:ident) => {
        $crate::concat_idents!(make_classify=Make,$name,$classify{
             #[derive(Default, Debug, Clone, Copy)]
             pub struct make_classify;

             impl $make_trait for make_classify {
                type $classify = __Classify;

                fn name(&self) -> &'static str {
                    stringify!($name)
                }

                fn make(&self, args: Args) -> Result<Self::$classify, Error> {
                    make(args)
                }
            }
         });
    };
    ($make_trait:ident,$classify:ident,$name:ident) => {
        $crate::concat_idents!(classify = $name,$classify {
            type __Classify = classify;
        });
        $crate::make_impl!(@internal $make_trait,$classify,$name);
    };
    ($make_trait:ident,$classify:ident,$name:ident,$mode:ident,$($(#[$meta:meta])* $vis:vis $field:ident:$ty:ty),* $(,)?) => {
        $crate::concat_idents!(classify = $name,$classify {
            type __Classify = classify;
            $crate::config!($name,$mode,$($(#[$meta])* $vis $field : $ty),*);
        });
        $crate::make_impl!(@internal $make_trait,$classify,$name);
    };
}
