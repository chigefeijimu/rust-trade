use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountBalance {
    pub free: u128,
    pub reserved: u128,
    pub total: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferDetails {
    pub from: String,
    pub to: String,
    pub amount: u128,
    pub block_hash: String,
    pub block_number: u32,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockEvent {
    pub block_number: u32,
    pub block_hash: String,
    pub event_index: u32,
    pub event_type: String,
    pub params: Vec<String>,
}