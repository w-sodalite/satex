use satex_core::Error;
use satex_serve::App;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = App::detect()?;
    app.run().await
}
