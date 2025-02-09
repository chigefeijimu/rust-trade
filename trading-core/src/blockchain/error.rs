use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlockchainError {
    #[error("Connection error: {0}")]
    ConnectionError(String),
    
    #[error("Invalid address")]
    InvalidAddress,
    
    #[error("Account not found")]
    AccountNotFound,
    
    #[error("Storage error: {0}")]
    StorageError(String),
    
    #[error("Decode error: {0}")]
    DecodeError(String),

    #[error("Transaction error: {0}")]
    TransactionError(String),

    #[error("Query error: {0}")]
    QueryError(String),
}