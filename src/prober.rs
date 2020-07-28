use std::time::Duration;

pub struct ProbeTarget {
    pub address: String,
    pub statsd_key: String,
    pub interval: Duration,
}
