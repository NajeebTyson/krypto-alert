use crate::core::alert::Alert;
use crate::core::Alerts;
use crate::error::AppError;

use std::fs;

const ALERTS_FILEPATH: &str = "./data/alerts.json";

/// Write alerts to json file
pub fn write_data(alerts: &Alerts) -> Result<(), AppError> {
    info!("store::write_data writing alerts to file");
    let data_str = serde_json::to_string(&alerts.get_alerts_list())?;
    fs::write(ALERTS_FILEPATH, data_str)?;
    Ok(())
}

/// Read alerts from json file
pub fn read_data() -> Result<Vec<Alert>, AppError> {
    info!("store::read_data reading alerts to file");
    let data = fs::read(ALERTS_FILEPATH)?;
    let data: Vec<Alert> = serde_json::from_slice(&data)?;
    Ok(data)
}
