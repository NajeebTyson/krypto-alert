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

use crate::core::worker::Worker;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    logger::init_logger();

    let alerts = Arc::new(Mutex::new(core::init()));
    let mut worker = Worker::init();

    let local_alerts = alerts.clone();
    let worker_res = worker.start(move || {
        let mut alerts = local_alerts.lock().unwrap();
        alerts.refresh();
    });

    if let Err(err) = worker_res {
        panic!("krypto-alert::main Error: {}", err.to_string());
    }

    thread::sleep(Duration::from_secs(260));
    println!("alerts: {:?}", alerts);

    worker.stop();
}
