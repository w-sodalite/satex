use satex::config::Config;
use satex::registry::Registry;
use satex::App;
use satex_core::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = Config::from_yaml("./examples/layer/cors/cors.yaml")?;
    let registry = Registry::default();
    App::new("Satex - Cors", config, registry).run().await
}
