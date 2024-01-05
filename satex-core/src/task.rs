use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use pin_project_lite::pin_project;

use crate::{satex_error, Error};

pin_project! {
    pub struct JoinHandle<T>{
        #[pin]
        inner:tokio::task::JoinHandle<T>
    }
}

impl<T> JoinHandle<T> {
    pub fn tokio(inner: tokio::task::JoinHandle<T>) -> Self {
        Self { inner }
    }

    pub fn abort(&self) {
        self.inner.abort()
    }

    pub fn is_finished(&self) -> bool {
        self.inner.is_finished()
    }
}

impl<T> Future for JoinHandle<T> {
    type Output = Result<T, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project().inner.poll(cx).map_err(|e| satex_error!(e))
    }
}

pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    JoinHandle::tokio(tokio::spawn(future))
}
