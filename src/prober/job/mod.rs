pub mod socket;

use futures::lock::MutexGuard;
use snafu::{ResultExt, Snafu};
use tokio::{io, net};

#[derive(Debug, Snafu)]
pub enum Error {
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

    async fn resolve<'a>(
        &self,
        socket_factory: &'a socket::Factory,
    ) -> Result<MutexGuard<'a, net::UdpSocket>, Error> {
        let mut last_socket_err = None;

        for address in net::lookup_host(&self.address).await.context(LookupHost)? {
            match socket_factory.get(address).await {
                Err(e) => last_socket_err = Some(e),
                Ok(mutex_guard) => return Ok(mutex_guard),
            }
        }

        if let Some(e) = last_socket_err {
            return Err(Error::SocketFactory { source: e });
        }
        Err(Error::LookupHostEmpty)
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub async fn probe(&self, socket_factory: &socket::Factory) -> Result<(), Error> {
        let mut socket_guard = self.resolve(&socket_factory).await?;
        let socket = &mut *socket_guard;

        socket
            .send_to(&self.payload, &self.address)
            .await
            .context(SendPayload)?;
        Ok(())
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
