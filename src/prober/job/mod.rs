mod socket;

use snafu::{ResultExt, Snafu};
use tokio::{io, net};

#[derive(Debug, Snafu)]
enum Error {
    LookupHost { source: io::Error },
    LookupHostEmpty,
    SendPayload { source: io::Error },
    SocketFactory { source: socket::Error },
}

fn build_payload(statsd_key: &str) -> Box<[u8]> {
    format!("{}:1|c", statsd_key)
        .into_bytes()
        .into_boxed_slice()
}

pub struct Job {
    address: String,
    payload: Box<[u8]>,
}

impl Job {
    pub fn new(address: String, payload: &str) -> Self {
        Self {
            address,
            payload: build_payload(payload),
        }
    }

    async fn send_payload(
        &self,
        socket: &mut net::UdpSocket,
        address: std::net::SocketAddr,
    ) -> Result<(), Error> {
        socket
            .send_to(&self.payload, address)
            .await
            .context(SendPayload)?;
        Ok(())
    }

    async fn probe(&self, socket_factory: socket::Factory) -> Result<(), Error> {
        let mut last_socket_err = None;

        for address in net::lookup_host(&self.address).await.context(LookupHost)? {
            match socket_factory.get(address).await {
                Err(e) => last_socket_err = Some(e),
                Ok(mut socket) => return self.send_payload(&mut *socket, address).await,
            }
        }

        if let Some(e) = last_socket_err {
            return Err(Error::SocketFactory { source: e });
        }
        Err(Error::LookupHostEmpty)
    }
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
