use std::time::Duration;

#[derive(Debug, PartialEq)]
pub struct ProbeTarget {
    pub address: String,
    pub statsd_key: String,
    pub interval: Duration,
}
