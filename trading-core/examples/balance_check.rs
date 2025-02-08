use trading_core::blockchain::{BlockchainManager, error::BlockchainError};

#[tokio::main]
async fn main() -> Result<(), BlockchainError> {
    let blockchain = BlockchainManager::new("ws://127.0.0.1:9944").await?;
    
    let address = blockchain.get_test_account();
    println!("Test account address: {}", address);
    
    let balance = blockchain.get_account_balance(&address).await?;
    println!("Balance:");
    println!("  Free: {} planck", balance.free);
    println!("  Reserved: {} planck", balance.reserved);
    println!("  Total: {} planck", balance.total);
    
    Ok(())
}