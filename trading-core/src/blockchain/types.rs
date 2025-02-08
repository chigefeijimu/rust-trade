use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountBalance {
    pub free: u128,
    pub reserved: u128,
    pub total: u128,
}