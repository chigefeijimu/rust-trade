// examples/transfer_test.rs
use trading_core::blockchain::{BlockchainManager, error::BlockchainError};
use sp_keyring::AccountKeyring;

#[tokio::main]
async fn main() -> Result<(), BlockchainError> {
    // 1. 连接到本地节点
    println!("Connecting to local node...");
    let blockchain = BlockchainManager::new("ws://127.0.0.1:9944").await?;
    
    // 2. 准备账户
    let alice = AccountKeyring::Alice.pair();
    let bob_address = AccountKeyring::Bob.to_account_id().to_string();
    println!("Bob's address: {}", bob_address);

    // 3. 查询转账前的历史
    println!("\nBefore transfer - Checking Alice's transfer history...");
    let alice_address = AccountKeyring::Alice.to_account_id().to_string();
    let history_before = blockchain.get_transfer_history(&alice_address).await?;
    println!("Found {} historical events", history_before.len());
    for event in history_before {
        println!("Block #{} - {} event:", event.block_number, event.event_type);
        println!("  Parameters: {:?}", event.params);
    }

    // 4. 执行转账
    let transfer_amount = 100_000_000_000_000; // 0.1 DOT
    println!("\nTransferring {} planck from Alice to Bob...", transfer_amount);
    
    let result = blockchain.transfer(alice, &bob_address, transfer_amount).await?;
    println!("Transfer successful!");
    println!("Transaction details:");
    println!("  From: {}", result.from);
    println!("  To: {}", result.to);
    println!("  Amount: {}", result.amount);
    println!("  Block hash: {}", result.block_hash);
    println!("  Block number: {}", result.block_number);

    // 5. 等待几秒确保交易完成
    tokio::time::sleep(tokio::time::Duration::from_secs(6)).await;

    // 6. 查询转账后的历史
    println!("\nAfter transfer - Checking Alice's transfer history...");
    let history_after = blockchain.get_transfer_history(&alice_address).await?;
    println!("Found {} historical events", history_after.len());
    for event in history_after {
        println!("Block #{} - {} event:", event.block_number, event.event_type);
        println!("  Parameters: {:?}", event.params);
    }

    Ok(())
}