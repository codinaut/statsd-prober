use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use humantime::{parse_duration, DurationError};
use itertools::Itertools;
use snafu::{ResultExt, Snafu};
use std::ffi::OsString;
use std::time::Duration;

pub struct Configuration {
    pub interval: Duration,
    pub targets: Vec<ProbeTarget>,
}

#[derive(Debug, PartialEq)]
pub struct ProbeTarget {
    pub address: String,
    pub statsd_key: String,
}

#[derive(Debug, Snafu)]
pub enum Error {
    ParseDuration { source: DurationError },
}

fn parse_targets<'v, V>(values: V) -> Vec<ProbeTarget>
where
    V: Iterator<Item = &'v str>,
{
    values
        .into_iter()
        .tuples()
        .map(|(address, statsd_key)| ProbeTarget {
            address: address.to_string(),
            statsd_key: statsd_key.to_string(),
        })
        .collect()
}

pub fn parse_args<I, T>(args: I) -> Result<Configuration, Error>
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
                .takes_value(true)
                .multiple(true)
                .number_of_values(2),
        )
        .arg(
            Arg::with_name("interval")
                .short("i")
                .help("Probing interval")
                .takes_value(true)
                .default_value("1s"),
        )
        .get_matches_from(args);

    Ok(Configuration {
        interval: parse_duration(matches.value_of("interval").unwrap()).context(ParseDuration)?,
        targets: matches
            .values_of("target")
            .map_or(vec![], |values| parse_targets(values)),
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use fake::faker;
    use fake::Fake;

    #[test]
    fn parse_first_target() {
        let address = faker::lorem::en::Word().fake();
        let statsd_key = faker::lorem::en::Word().fake();
        let arg = parse_args(&["app", "-t", address, statsd_key]).unwrap();
        assert_eq!(
            arg.targets[0],
            ProbeTarget {
                address: address.to_string(),
                statsd_key: statsd_key.to_string(),
            }
        )
    }

    #[test]
    fn parse_next_target() {
        let address = faker::lorem::en::Word().fake();
        let statsd_key = faker::lorem::en::Word().fake();
        let arg = parse_args(&["app", "-t", "a", "b", "-t", address, statsd_key]).unwrap();
        assert_eq!(
            arg.targets[1],
            ProbeTarget {
                address: address.to_string(),
                statsd_key: statsd_key.to_string(),
            }
        )
    }

    #[test]
    fn duration() {
        let arg = parse_args(&["app", "-i", "500ms"]).unwrap();
        assert_eq!(arg.interval, Duration::from_millis(500))
    }
}
