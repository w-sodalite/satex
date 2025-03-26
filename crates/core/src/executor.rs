use hyper::rt::Executor;

#[derive(Debug, Clone, Copy, Default)]
pub struct SpawnLocalExecutor {}

impl SpawnLocalExecutor {
    pub fn new() -> Self {
        Self {}
    }
}

impl<F> Executor<F> for SpawnLocalExecutor
where
    F: Future + 'static,
    F::Output: 'static,
{
    fn execute(&self, future: F) {
        tokio::task::spawn_local(future);
    }
}
