#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate config;

mod core;
mod error;
mod logger;
mod settings;
mod types;
mod utils;

use crate::core::alert::{Alert, AlertType};
use crate::core::api;
use crate::core::symbol::Symbol;

use futures::executor::block_on;

#[tokio::main]
async fn main() {
    logger::init_logger();

    // let res = block_on(core::api::get_symbol_price("ETHUSDTT"));
    // println!("res: {:#?}", res);
    // block_on(core::api::get_all_symbols_price());
    // let symbol = Symbol::new("ETHUSDT");
    // symbol.set_alert()
    // Alert::new(Symbol { name: "ETHUSDT".to_string(), current_price: 0.0 }, AlertType::PriceAbove(4200.));
    let mut alerts = core::init();
    alerts.create_alert("ETHUSDT", AlertType::PriceAbove(4126.));
    alerts.create_alert("DOGEUSDT", AlertType::ChangeOver(1.));
    alerts.create_alert("DOGEUSDT", AlertType::ChangeUnder(1.));
    println!("alerts: {:?}", alerts);
}
