#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate config;
#[macro_use]
extern crate log;

mod core;
mod error;
mod logger;
mod settings;
mod types;
mod utils;

use crate::core::{alert::AlertType, worker::Worker};

use futures::executor::block_on;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    logger::init_logger();

    let mut alerts = Arc::new(Mutex::new(core::init()));
    let mut worker = Worker::init();

    alerts
        .lock()
        .unwrap()
        .create_alert("DOGEUSDT", AlertType::ChangeOver(1.), true);
    alerts
        .lock()
        .unwrap()
        .create_alert("ETHUSDT", AlertType::ChangeOver(1.), true);

    let local_alerts = alerts.clone();
    worker.start(move || {
        let mut alerts = local_alerts.lock().unwrap();
        alerts.refresh();
    });

    thread::sleep(Duration::from_secs(2));
    println!("alerts: {:?}", alerts);

    worker.stop();
}
