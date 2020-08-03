use lazy_futuristic::Lazy;
use lazy_futuristic::ValueOrSetter::*;
use futures::lock::{Mutex, MutexGuard};
use snafu::{ResultExt, Snafu};
use std::net::SocketAddr;
use tokio::io;
use tokio::net::UdpSocket;

#[derive(Debug, Snafu)]
pub enum Error {
    UnknownSocketType,
    Bind { source: io::Error },
}

#[derive(Default)]
pub struct Factory {
    ipv4_socket: Lazy<Mutex<UdpSocket>>,
    ipv6_socket: Lazy<Mutex<UdpSocket>>,
}

impl Factory {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn get<'l>(&'l self, address: SocketAddr) -> Result<MutexGuard<'l, UdpSocket>, Error> {
        if address.is_ipv4() {
            let socket = match self.ipv4_socket.get_or_set().await {
                Value(value) => value,
                Setter(setter) => {
                    let s = UdpSocket::bind("0.0.0.0:0").await.context(Bind)?;
                    setter.set(Mutex::new(s))
                }
            };
            return Ok(socket.lock().await);
        }

        if address.is_ipv6() {
            let socket = match self.ipv4_socket.get_or_set().await {
                Value(value) => value,
                Setter(setter) => {
                    let s = UdpSocket::bind("[::]:0").await.context(Bind)?;
                    setter.set(Mutex::new(s))
                }
            };
            return Ok(socket.lock().await);
        }

        Err(Error::UnknownSocketType)
    }
}
