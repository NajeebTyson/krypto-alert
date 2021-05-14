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
// use crate::core::api::

use futures::executor::block_on;
// use async_std::task;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// #[tokio::main]
fn main() {
    logger::init_logger();

    // let res = block_on(core::api::get_symbol_price("ETHUSDT"));
    // println!("res: {:#?}", res);
    // block_on(core::api::get_all_symbols_price());
    // let symbol = Symbol::new("ETHUSDT");
    // symbol.set_alert()
    // Alert::new(Symbol { name: "ETHUSDT".to_string(), current_price: 0.0 }, AlertType::PriceAbove(4200.));

    let mut alerts = Arc::new(Mutex::new(core::init()));
    let mut worker = Worker::init();
    let local_alerts = alerts.clone();
    worker.start(move || {
        let mut alerts = local_alerts.lock().unwrap();
        alerts.refresh();
    });

    // .create_alert("ETHUSDT", AlertType::PriceAbove(4126.));
    // alerts.create_alert("DOGEUSDT", AlertType::ChangeOver(1.));
    // // alerts.create_alert("DOGEUSDT", AlertType::ChangeUnder(1.));

    println!("alerts: {:?}", alerts);

    thread::sleep(Duration::from_secs(1));
    worker.stop();
}
