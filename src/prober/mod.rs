mod job;

use super::args::Configuration;
use futures::lock::Mutex;
use std::sync::Arc;
use std::time::Duration;
use tokio::time;

pub struct Prober {
    interval: Duration,
    jobs: Arc<[Mutex<job::Job>]>,
    socket_factory: Arc<job::socket::Factory>,
}

impl Prober {
    pub fn new(configuration: Configuration) -> Self {
        Prober {
            interval: configuration.interval,
            jobs: Arc::from(
                configuration
                    .targets
                    .iter()
                    .map(|target| {
                        Mutex::new(job::Job::new(target.address.clone(), &target.statsd_key))
                    })
                    .collect::<Vec<Mutex<job::Job>>>()
                    .into_boxed_slice(),
            ),
            socket_factory: Arc::new(job::socket::Factory::new())
        }
    }

    pub async fn probe_all_periodically(&self) {
        let mut ticker = time::interval(self.interval);

        loop {
            ticker.tick().await;
            self.probe_all().await;
        }
    }

    async fn probe_all(&self) {
        for (i, _) in self.jobs.iter().enumerate() {
            let jobs = self.jobs.clone();
            let socket_factory = self.socket_factory.clone();

            tokio::task::spawn(async move {
                if let Some(job_guard) = &jobs[i].try_lock() {
                    let job = &*job_guard;
                    job.probe(&socket_factory).await.unwrap();
                }
            });
        }
    }
}
