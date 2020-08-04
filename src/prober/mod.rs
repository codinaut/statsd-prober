mod job;

use super::args::Configuration;
use std::sync::Arc;
use std::time::Duration;
use tokio::time;
use tracing::{info, warn};
use humantime::format_duration;

pub struct Prober {
    interval: Duration,
    jobs: Arc<[job::Job]>,
    socket_factory: Arc<job::socket::Factory>,
}

impl Prober {
    pub fn new(configuration: Configuration) -> Self {
        info!(interval = format_duration(configuration.interval).to_string().as_str(), "Initialize prober");
        Prober {
            interval: configuration.interval,
            jobs: Arc::from(
                configuration
                    .targets
                    .iter()
                    .map(|target| {
                        info!(address = target.address.as_str(), statsd_key = target.statsd_key.as_str(), "Discover target");
                        job::Job::new(target.address.clone(), &target.statsd_key)
                    })
                    .collect::<Vec<job::Job>>()
                    .into_boxed_slice(),
            ),
            socket_factory: Arc::new(job::socket::Factory::new())
        }
    }

    pub async fn probe_all_periodically(&self) {
        let mut ticker = time::interval(self.interval);
        loop {
            ticker.tick().await;
            info!("Tick");
            self.probe_all().await;
        }
    }

    async fn probe_all(&self) {
        for (i, _) in self.jobs.iter().enumerate() {
            let jobs = self.jobs.clone();
            let socket_factory = self.socket_factory.clone();

            tokio::task::spawn(async move {
                let job = &jobs[i];
                match job.run_probe(&socket_factory).await {
                    Err(e) => warn!(address = job.address(), "Probe error: {}", e),
                    Ok(()) => info!(address = job.address(), "Probe ok"),
                }
            });
        }
    }
}
