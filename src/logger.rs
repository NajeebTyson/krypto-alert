use env_logger::Builder;
use log::LevelFilter;

use crate::settings::SETTINGS;

/// Initialize logging
/// LevelFilter set to anyone below the list, logs all below that will execute
/// e.g. if LevelFilter set to info, then all info, warn and error will execute
///
/// trace!("some trace log");
/// debug!("some debug log");
/// info!("some information log");
/// warn!("some warning log");
/// error!("some error log");
pub fn init_logger() {
    let level_str: String = SETTINGS.logging.level.clone();
    let level = match level_str.as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        "off" => LevelFilter::Off,
        _ => LevelFilter::Debug,
    };
    Builder::default().filter_level(level).init();
}
