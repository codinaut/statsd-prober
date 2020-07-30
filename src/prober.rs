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

fn build_payload(statsd_key: &str) -> Box<[u8]> {
    format!("{}:1|c", statsd_key)
        .into_bytes()
        .into_boxed_slice()
}

#[cfg(test)]
mod test {
    use super::*;
    use fake::faker;
    use fake::Fake;

    #[test]
    fn build_payload_ok() {
        let statsd_key = faker::lorem::en::Word().fake();
        assert_eq!(
            build_payload(statsd_key),
            format!("{}:1|c", statsd_key)
                .into_bytes()
                .into_boxed_slice()
        );
    }
}
