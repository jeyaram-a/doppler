use std::{
    borrow::BorrowMut,
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    sync::{Arc, Mutex},
    time::Instant,
};
use tokio::time::{self, sleep, Duration};

use super::registration::Registration;

struct MonitoringJob {
    client_id: String,
    last_ping_timestamp: Arc<Mutex<Instant>>,
}

impl MonitoringJob {
    fn new(client_id: String) -> MonitoringJob {
        MonitoringJob {
            client_id: client_id,
            last_ping_timestamp: Arc::new(Mutex::new(Instant::now())),
        }
    }

    async fn monitor(&self) {
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            let mut boxxed_last_ping_interval = self
                .last_ping_timestamp
                .as_ref()
                .borrow_mut()
                .lock()
                .unwrap();
            *boxxed_last_ping_interval = Instant::now();
            print!("executed {:?}", boxxed_last_ping_interval);
        }
    }
}

struct Manager<'a> {
    registrations: HashMap<&'a str, Registration<'a>>,
    ping_records: HashMap<&'a str, MonitoringJob>,
}

impl<'a> Manager<'a> {
    pub fn new() -> Manager<'a> {
        Manager {
            registrations: HashMap::new(),
            ping_records: HashMap::new(),
        }
    }

    pub async fn register(&mut self, registration: Registration<'a>) {
        let registration_id: String = registration.id.to_string();
        self.registrations.insert(registration.id, registration);
        let job = Arc::new(MonitoringJob::new(registration_id));

        tokio::spawn(async move {
            job.as_ref().monitor().await;
        });
        println!("done {:?}", Instant::now())
    }
}

#[cfg(test)]
mod tests {
    use tokio::time::{self, sleep, Duration};

    #[tokio::test]
    async fn it_works() {
        let mut manager = super::Manager::new();
        manager.register(super::Registration::new()).await;
        tokio::time::sleep(Duration::from_millis(5000)).await;
    }
}
