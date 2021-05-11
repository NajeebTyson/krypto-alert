use crate::types::*;

use uuid::Uuid;

/// Generate new alert id
pub fn generate_new_alert_id() -> AlertId {
    Uuid::new_v4().to_string()
}
