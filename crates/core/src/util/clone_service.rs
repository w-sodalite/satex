use futures_util::future::LocalBoxFuture;
use std::fmt;
use std::task::{Context, Poll};
use tower::layer::{layer_fn, LayerFn};
use tower::{Service, ServiceExt};

pub struct SyncBoxCloneService<T, U, E>(
    Box<
        dyn CloneService<T, Response=U, Error=E, Future=LocalBoxFuture<'static, Result<U, E>>>
            + Send
            + Sync,
    >,
);

impl<T, U, E> SyncBoxCloneService<T, U, E> {
    /// Create a new `SyncBoxCloneService`.
    pub fn new<S>(inner: S) -> Self
    where
        S: Service<T, Response = U, Error = E> + Clone + Send + Sync + 'static,
        S::Future: 'static,
    {
        let inner = inner.map_future(|f| Box::pin(f) as _);
        SyncBoxCloneService(Box::new(inner))
    }

    /// Returns a [`Layer`] for wrapping a [`Service`] in a [`SyncBoxCloneService`]
    /// middleware.
    ///
    /// [`Layer`]: crate::Layer
    pub fn layer<S>() -> LayerFn<fn(S) -> Self>
    where
        S: Service<T, Response = U, Error = E> + Clone + Send + Sync + 'static,
        S::Future: 'static,
    {
        layer_fn(Self::new)
    }
}

impl<T, U, E> Service<T> for SyncBoxCloneService<T, U, E> {
    type Response = U;
    type Error = E;
    type Future = LocalBoxFuture<'static, Result<U, E>>;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), E>> {
        self.0.poll_ready(cx)
    }

    #[inline]
    fn call(&mut self, request: T) -> Self::Future {
        self.0.call(request)
    }
}

impl<T, U, E> Clone for SyncBoxCloneService<T, U, E> {
    fn clone(&self) -> Self {
        Self(self.0.clone_box())
    }
}

trait CloneService<R>: Service<R> {
    fn clone_box(
        &self,
    ) -> Box<
        dyn CloneService<R, Response = Self::Response, Error = Self::Error, Future = Self::Future>
            + Send
            + Sync,
    >;
}

impl<R, T> CloneService<R> for T
where
    T: Service<R> + Send + Clone + Sync + 'static,
{
    fn clone_box(
        &self,
    ) -> Box<
        dyn CloneService<R, Response = T::Response, Error = T::Error, Future = T::Future>
            + Send
            + Sync,
    > {
        Box::new(self.clone())
    }
}

impl<T, U, E> fmt::Debug for SyncBoxCloneService<T, U, E> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("SyncBoxCloneService").finish()
    }
}
