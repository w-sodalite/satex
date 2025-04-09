use satex::App;
use satex::config::Config;
use satex::registry::Registry;
use satex::watch::ConfigFileWatchEvents;
use satex_core::Error;
use std::collections::VecDeque;
use std::env::current_dir;
use std::path::PathBuf;
use std::time::Duration;

///
/// Satex服务名称
///
const SATEX: &str = "Satex";

///
/// 配置文件名称
///
const SATEX_YAML: &str = "satex.yaml";

///
/// 配置文件监控扫描间隔时间
///
const WATCH_FILE_INTERVAL: Duration = Duration::from_secs(10);

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    let registry = Registry::default();
    let path = get_config_path()?;
    let events = ConfigFileWatchEvents::events(registry.clone(), path.clone(), WATCH_FILE_INTERVAL);
    let config = Config::from_yaml(path)?;
    App::new(SATEX, config, registry)
        .with_events(events)
        .run()
        .await
}

/// 获取配置文件路径
///
/// 该函数尝试从命令行参数中解析配置文件路径。
/// 如果未提供配置文件路径，它将尝试使用当前目录下的默认配置文件。
///
/// # Returns
/// - `Ok(PathBuf)` 如果成功获取配置文件路径
/// - `Err(Error)` 如果无法获取配置文件路径
///
fn get_config_path() -> Result<PathBuf, Error> {
    let mut args = std::env::args()
        .map(|it| it.trim().to_string())
        .collect::<VecDeque<_>>();
    let path = loop {
        if let Some(arg) = args.pop_front() {
            if arg == "-c" || arg == "--config" {
                if let Some(value) = args.pop_front() {
                    break Some(value);
                }
            }
        } else {
            break None;
        }
    };
    match path {
        Some(path) => Ok(PathBuf::from(path)),
        None => current_dir()
            .map(|dir| dir.join(SATEX_YAML))
            .map_err(Error::new),
    }
}
