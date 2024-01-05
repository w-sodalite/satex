use satex_core::{config::args::Args, Error};

use crate::{MakeRouteServiceLayer, __layer};

use super::layer::SetPathLayer;

__layer! {
    SetPath,
    path:String
}

fn make(args: Args) -> Result<SetPathLayer, Error> {
    let config = Config::try_from(args)?;
    Ok(SetPathLayer::new(config.path))
}
