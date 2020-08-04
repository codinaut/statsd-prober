mod args;
mod prober;

use snafu::{ResultExt, Snafu};
use std::{env, fmt};
use tokio;
use tracing::Level;
use tracing_subscriber;

#[derive(Snafu)]
enum Error {
    #[snafu(display("Unable to parse argument: {}", source))]
    ParseArgument { source: args::Error },

    #[snafu(display("Unable to set tracing global subscriber: {}", source))]
    SetTracingGlobalSubscriber {
        source: tracing::dispatcher::SetGlobalDefaultError,
    },
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::subscriber::set_global_default(
        tracing_subscriber::fmt()
            .with_max_level(Level::TRACE)
            .finish(),
    )
    .context(SetTracingGlobalSubscriber)?;

    prober::Prober::new(args::parse_args(env::args()).context(ParseArgument)?)
        .probe_all_periodically()
        .await;
    Ok(())
}
