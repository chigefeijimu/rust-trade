pub mod types;
pub mod error;

use std::str::FromStr;
use error::BlockchainError;
use subxt::{utils::AccountId32, OnlineClient, PolkadotConfig};
use sp_keyring::AccountKeyring;
use codec::{Decode, Encode};

pub struct BlockchainManager {
    client: OnlineClient<PolkadotConfig>,
}

#[derive(Debug, Decode)]
struct AccountInfo {
    data: AccountData,
}

#[derive(Debug, Decode)]
struct AccountData {
    free: u128,
    reserved: u128,
}

impl BlockchainManager {
    pub async fn new(node_url: &str) -> Result<Self, BlockchainError> {
        let client = OnlineClient::<PolkadotConfig>::from_url(node_url)
            .await
            .map_err(|e| BlockchainError::ConnectionError(e.to_string()))?;
            
        Ok(Self { client })
    }

    pub fn get_client(&self) -> &OnlineClient<PolkadotConfig> {
        &self.client
    }

    pub fn get_test_account(&self) -> String {
        let account = AccountKeyring::Alice.to_account_id();
        account.to_string()
    }

    pub async fn get_account_balance(&self, address: &str) -> Result<types::AccountBalance, BlockchainError> {
        let storage = self.client.storage();
        
        let at_block = storage.at_latest().await
            .map_err(|e| BlockchainError::StorageError(e.to_string()))?;
        
        let account_id = AccountId32::from_str(address)
            .map_err(|_| BlockchainError::InvalidAddress)?;
        
        let maybe_account = at_block
            .fetch(&subxt::dynamic::storage("System", "Account", vec![account_id]))
            .await
            .map_err(|e| BlockchainError::StorageError(e.to_string()))?;
    
        match maybe_account {
            Some(account_data) => {
                let account_info = AccountInfo::decode(&mut account_data.encoded())
                    .map_err(|e| BlockchainError::DecodeError(e.to_string()))?;
                
                Ok(types::AccountBalance {
                    free: account_info.data.free,
                    reserved: account_info.data.reserved,
                    total: account_info.data.free + account_info.data.reserved,
                })
            }
            None => Err(BlockchainError::AccountNotFound),
        }
    }
    
    pub async fn transfer(
        &self,
        from_pair: subxt::ext::sp_core::sr25519::Pair,
        to_address: &str,
        amount: u128
    ) -> Result<types::TransferDetails, BlockchainError> {
        println!("Step 1: Converting addresses...");
        let to_account = AccountId32::from_str(to_address)
            .map_err(|_| BlockchainError::InvalidAddress)?;
    
        println!("Step 2: Creating signer...");
        let pair_signer = subxt::tx::PairSigner::new(from_pair);
    
        println!("Step 3: Building transaction...");
        let dest = subxt::dynamic::Value::named_composite([
            ("Id", subxt::dynamic::Value::from_bytes(to_account.encode()))
        ]);
    
        let call = subxt::dynamic::tx(
            "Balances",
            "transfer_allow_death",
            vec![
                dest,
                subxt::dynamic::Value::u128(amount),
            ],
        );
    
        println!("Step 4: Submitting transaction...");
        match self.client
            .tx()
            .sign_and_submit_default(&call, &pair_signer)
            .await {
                Ok(_) => println!("Transaction submitted successfully"),
                Err(e) => {
                    println!("Transaction submission error: {:?}", e);
                    return Err(BlockchainError::TransactionError(e.to_string()));
                }
            };
    
        println!("Step 5: Getting latest block...");

        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        
        let block = self.client
            .blocks()
            .at_latest()
            .await
            .map_err(|e| BlockchainError::QueryError(e.to_string()))?;
    
        let block_hash = block.hash();
        let block_number = block.number();
    
        println!("Step 6: Transaction completed.");
        Ok(types::TransferDetails {
            from: pair_signer.account_id().to_string(),
            to: to_address.to_string(),
            amount,
            block_hash: block_hash.to_string(),
            block_number,
            success: true,
        })
    }
    
    pub async fn get_transfer_history(&self, address: &str) -> Result<Vec<types::BlockEvent>, BlockchainError> {
        let mut events = Vec::new();
        let account_id = AccountId32::from_str(address)
            .map_err(|_| BlockchainError::InvalidAddress)?;
    
        let latest_block = self.client
            .blocks()
            .at_latest()
            .await
            .map_err(|e| BlockchainError::QueryError(e.to_string()))?;
    
        let latest_number = latest_block.number();
        let start_block = latest_number.saturating_sub(100);
    
        for number in (start_block..=latest_number).rev() {
            if let Ok(block) = self.client.blocks().at(latest_block.hash()).await {
                if let Ok(events_result) = block.events().await {
                    for (event_idx, event) in events_result.iter().enumerate() {
                        if let Ok(event) = event {
                            if event.pallet_name() == "Balances" && 
                               (event.variant_name() == "Transfer" || 
                                event.variant_name() == "Deposit" || 
                                event.variant_name() == "Withdraw") {
                                
                                let mut params = Vec::new();
                                while let Ok(field) = event.field_values() {
                                    params.push(field.to_string());
                                }
    
                                if params.iter().any(|p| p.contains(&account_id.to_string())) {
                                    events.push(types::BlockEvent {
                                        block_number: number,
                                        block_hash: block.hash().to_string(),
                                        event_index: event_idx as u32,
                                        event_type: event.variant_name().to_string(),
                                        params,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    
        Ok(events)
    }
}