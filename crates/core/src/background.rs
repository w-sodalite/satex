use async_trait::async_trait;
use std::sync::Arc;
use tracing::info;

///
/// 后台任务
///
#[async_trait]
pub trait BackgroundTask {
    /// 执行任务
    async fn run(&self);
}

pub async fn background_task<T: BackgroundTask + Send + Sync + 'static>(
    name: impl ToString,
    task: Arc<T>,
) {
    info!("starting background task: {}", name.to_string());
    task.run().await;
}
