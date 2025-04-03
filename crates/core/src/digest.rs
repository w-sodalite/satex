pub trait Digester<M> {
    fn digest(&self, input: &M) -> Vec<u8>;
}
