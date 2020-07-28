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
