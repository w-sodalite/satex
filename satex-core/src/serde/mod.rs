pub mod http;
pub mod regex;
pub mod tot;
pub mod tracing;

#[macro_export]
macro_rules! serde_with {
    ($src:ident, $serde_mode:literal, [$($derive:ty),*]) => {
        $crate::concat_idents!(dst_name=Serde,$src {
            #[derive(serde::Serialize, serde::Deserialize,$($derive),*)]
            pub struct dst_name(#[serde(with = $serde_mode)] $src);

            impl From<$src> for dst_name {
                fn from(value: $src) -> Self {
                    dst_name(value)
                }
            }

            impl From<dst_name> for $src {
                fn from(value: dst_name) -> Self {
                    value.0
                }
            }
        });


    };
}
