use crate::types::*;

/// Symbol struct which contains all the symbol related attributes
pub struct Symbol {
    pub name: PairName,
    current_price: Amount,
}

impl Symbol {
    /// Create new symbol
    pub fn new(symbol_name: PairName) -> Self {
        Symbol {
            name: symbol_name,
            current_price: 0.0,
        }
    }
}
