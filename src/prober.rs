#[derive(Debug, PartialEq)]
pub struct ProbeTarget {
    pub address: String,
    pub statsd_key: String,
}
