#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate config;

mod core;
mod error;
mod settings;
mod types;

use crate::core::alert::{Alert, AlertType};
use crate::core::symbol::Symbol;
use crate::settings::SETTINGS;

use futures::executor::block_on;

#[tokio::main]
async fn main() {
    println!("{:#?}", SETTINGS.settings);
    // let res = block_on(core::api::get_symbol_price("ETHUSDT"));
    // println!("res: {:#?}", res);
    // block_on(core::api::get_all_symbols_price());
    // let symbol = Symbol::new("ETHUSDT");
    // symbol.set_alert()
    // Alert::new(Symbol { name: "ETHUSDT".to_string(), current_price: 0.0 }, AlertType::PriceAbove(4200.));
}
