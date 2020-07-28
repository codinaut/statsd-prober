use crate::prober::ProbeTarget;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use itertools::Itertools;
use std::ffi::OsString;

pub struct Arguments {
    targets: Vec<ProbeTarget>,
}

fn parse_targets<V>(values: V) -> Vec<ProbeTarget>
where
    V: IntoIterator<Item = OsString>,
{
    values
        .into_iter()
        .tuples()
        .map(|(address, statsd_key)| ProbeTarget {
            address: address.into_string().unwrap(),
            statsd_key: statsd_key.into_string().unwrap(),
        })
        .collect()
}

pub fn parse_args<I, T>(args: I) -> Arguments
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
        .get_matches_from(args);

    Arguments {
        targets: matches
            .args
            .get("target")
            .map_or(vec![], |arg| parse_targets(arg.vals.clone())),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_first_target() {
        let arg = parse_args(&["app", "-t", "a", "b"]);
        assert_eq!(
            arg.targets[0],
            ProbeTarget {
                address: "a".to_string(),
                statsd_key: "b".to_string(),
            }
        )
    }

    #[test]
    fn parse_next_target() {
        let arg = parse_args(&["app", "-t", "a", "b", "-t", "c", "d"]);
        assert_eq!(
            arg.targets[1],
            ProbeTarget {
                address: "c".to_string(),
                statsd_key: "d".to_string(),
            }
        )
    }
}
