mod args;
mod prober;

use snafu::{ResultExt, Snafu};
use std::env;
use tokio;

#[derive(Debug, Snafu)]
enum Error {
    Argument { source: args::Error },
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    prober::Prober::new(args::parse_args(env::args()).context(Argument)?)
        .probe_all_periodically()
        .await;
    Ok(())
}
