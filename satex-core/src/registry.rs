#[macro_export]
macro_rules! registry {
    ($registry:ident,$arc_make:ty,[$($make:ty),* $(,)?]) => {

        use std::ops::DerefMut;
        use satex_core::apply::Apply;

        static STATUS: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

        $crate::lazy_static!{
            static ref GLOBAL: std::sync::RwLock<std::collections::HashMap<&'static str, $arc_make>> = std::sync::RwLock::new(std::collections::HashMap::new());
        }

        #[derive(Clone)]
        pub enum $registry {}

        impl $registry {

            fn builtin_makes() -> Vec<$arc_make> {
                vec![$(<$arc_make>::new(<$make>::default())),*]
            }

            fn add_builtin_makes() -> Result<(),$crate::Error>{
                if STATUS
                    .compare_exchange(false, true, std::sync::atomic::Ordering::Release, std::sync::atomic::Ordering::Acquire)
                    .is_ok()
                    {
                        // 初始化内置的Make
                        Self::write(|makes| {
                            Self::builtin_makes().into_iter().for_each(|make| {
                                makes.insert(make.name(), make);
                            });
                            Ok(())
                        })?;
                    };
                Ok(())
            }

            pub fn write<
                R,
                F: FnOnce(&mut std::collections::HashMap<&'static str, $arc_make>) -> Result<R,$crate::Error>,
            >(
                f: F,
            ) -> Result<R,$crate::Error> {
                     let mut makes = GLOBAL
                        .write()
                        .map_err(|e| $crate::satex_error!("Global make registry get lock error!"))?;
                     let makes = makes.deref_mut();
                     f(makes)
            }

            pub fn get(kind: &'_ str) -> Result<$arc_make,$crate::Error> {
                Self::add_builtin_makes()?;
                let makes = GLOBAL
                    .read()
                    .map_err(|e| $crate::satex_error!("Global make registry get lock error!"))?;
                makes.get(kind).map(|x| x.clone()).ok_or_else(|| $crate::satex_error!("Cannot find valid kind: {}",kind))
            }

            pub fn all() -> Result<std::collections::HashMap<&'static str, $arc_make>,$crate::Error>{
                Self::add_builtin_makes()?;
                let mut makes = GLOBAL
                        .read()
                        .map_err(|e| $crate::satex_error!("Global make registry get lock error!"))?;
                 Ok(makes.clone())
            }
        }

    };

    ($registry:ident, $arc_make:ty, $arc_item:ty, [$($make:ty),* $(,)?]) => {

        registry!($registry, $arc_make, [$($make),*]);

        impl $registry {

            pub fn make_many(items: &[satex_core::config::metadata::Metadata]) -> Result<Vec<$arc_item>, $crate::Error> {
                items.iter().try_fold(vec![], |targets, item| {
                    Self::get(item.kind())
                        .and_then(|make| make.make(item.args()))
                        .map(|target| targets.apply(|targets| targets.push(target)))
                })
            }

            pub fn make_single(item: &satex_core::config::metadata::Metadata) -> Result<$arc_item,$crate::Error> {
                Self::get(item.kind()).and_then(|make| make.make(item.args()))
            }
        }
    }
}
