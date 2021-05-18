use crate::core::alert::{AlertType, SimpleAlert};
use crate::core::api::SymbolResponse;
use crate::settings::SETTINGS;

use crate::error::AppError;
use notify_rust::Notification;

/// Notify the user about the alert
pub fn notify(symbol: &SymbolResponse, alert: &SimpleAlert) {
    let timeout: u32 = SETTINGS.settings.notification_timeout as u32 * 1000;
    let summary: String = format!("Krypto-Alert - {} ${}", symbol.symbol, symbol.price);
    let message: String = match alert.alert_type {
        AlertType::PriceAbove(price) => format!(
            "[{}] value is above {}. Current value: ${}",
            symbol.symbol, price, symbol.price
        ),
        AlertType::PriceBelow(price) => format!(
            "[{}] value is below {}. Current value: ${}",
            symbol.symbol, price, symbol.price
        ),
        AlertType::ChangeOver(percentage) => format!(
            "[{}] up {}%. Current Value: ${}",
            symbol.symbol, percentage, symbol.price
        ),
        AlertType::ChangeUnder(percentage) => format!(
            "[{}] down -{}%. Current Value: ${}",
            symbol.symbol, percentage, symbol.price
        ),
    };
    show_notification(&summary, &message, &timeout);
}

/// Create and show the notification
fn show_notification(summary: &str, message: &str, timeout: &u32) -> Result<(), AppError> {
    Notification::new()
        .summary(summary)
        .body(message)
        // .icon("firefox")
        // .appname("firefox")
        // .hint(Hint::Category("email".to_owned()))
        // .hint(Hint::Resident(true)) // this is not supported by all implementations
        .timeout(*timeout as i32) // this however is
        // .action("view", "View")
        // .action("ignore", "Ignore")
        .show()?;
    // .unwrap()
    // .wait_for_action(|action| match action {
    //     "default" => println!("you clicked \"default\""),
    //     "view" => println!("You clicked view"),
    //     "ignore" => println!("You clicked ignore"),
    //     // here "__closed" is a hard coded keyword
    //     "__closed" => println!("the notification was closed"),
    //     _ => (),
    // });
    Ok(())
}
