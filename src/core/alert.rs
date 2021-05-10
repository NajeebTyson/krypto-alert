use crate::core::symbol::Symbol;
use crate::types::*;

/// Enum for alert types
pub enum AlertType {
    PriceAbove(Amount),
    PriceBelow(Amount),
    ChangeOver(Percentage),
    ChangeUnder(Percentage),
    Change24hOver(Percentage),
    Change24hUnder(Percentage),
}

/// Main struct for alert
pub struct Alert {
    pub symbol: Symbol,
    pub alert_type: AlertType,
}

impl Alert {
    /// Create new alert
    pub fn new(pair_name: PairName, alert_type: AlertType) -> Self {
        Alert {
            symbol: Symbol::new(pair_name),
            alert_type,
        }
    }
}
