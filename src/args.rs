use crate::prober::ProbeTarget;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use snafu::{ResultExt, Snafu};
use std::ffi::OsString;
use std::time::Duration;
use std::ops::Deref;

#[derive(Debug, Snafu)]
pub enum Error {
    Clap { source: clap::Error },
    ParseDuration { source: humantime::DurationError },
}

pub struct Arguments {
    targets: Vec<ProbeTarget>,
}

fn parse_targets<'i, I, V>(indices: I, values: &V) -> Result<Vec<ProbeTarget>, Error>
where
    I: Iterator<Item = &'i usize>,
    V: Deref<Target = [OsString]>,
{
    let mut index = 0;

    Ok(indices
        .map(|u| {
            let probes = ProbeTarget {
                address: values[0].clone().into_string().unwrap(),
                statsd_key: values[1].clone().into_string().unwrap(),
                interval: Duration::from_secs(1),
            };

            index += u;
            probes
        })
        .collect())
}

pub fn parse_args<I, T>(args: I) -> Result<Arguments, Error>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let matches = App::new(crate_name!())
        .about(crate_description!())
        .author(crate_authors!())
        .version(crate_version!())
        .arg(
            Arg::with_name("target")
                .short("t")
                .help("Specify probe target(s)")
                .min_values(2)
                .max_values(3),
        )
        .get_matches_from_safe(args)
        .context(Clap)?;

    Ok(Arguments {
        targets: matches.args.get("target").map_or(Ok(vec![]), |arg| {
            parse_targets(arg.indices.iter(), &arg.vals)
        })?,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_first_target() {
        let arg = parse_args(&["app", "-t", "a", "b"]).unwrap();
        assert_eq!(arg.targets[0], ProbeTarget{
            address: "a".to_string(),
            statsd_key: "b".to_string(),
            interval: Duration::from_secs(1),
        })
    }
}
