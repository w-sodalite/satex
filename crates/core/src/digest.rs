use std::borrow::Cow;

pub trait Digester<M> {
    fn digest(&self, input: &M) -> Cow<[u8]>;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DefaultDigester;

impl<M> Digester<M> for DefaultDigester {
    fn digest(&self, _: &M) -> Cow<[u8]> {
        const BYTES: &[u8; 0] = &[];
        Cow::from(BYTES)
    }
}
