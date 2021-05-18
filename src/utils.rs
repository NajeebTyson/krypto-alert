use crate::types::*;

use uuid::Uuid;

/// Generate new alert id
pub fn generate_new_alert_id() -> AlertId {
    Uuid::new_v4().to_string()
}

/// Calculate percentage change of price
pub fn calc_percentage_change(old_price: &Amount, new_price: &Amount) -> Percentage {
    ((new_price - old_price) / old_price * 100.) as f32
}
