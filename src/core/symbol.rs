use crate::types::*;

/// Symbol struct which contains all the symbol related attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub name: PairName,
}

impl Symbol {
    /// Create new symbol
    pub fn new(symbol_name: &str) -> Self {
        Symbol {
            name: symbol_name.to_string(),
        }
    }
}
