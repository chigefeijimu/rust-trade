pub mod types;
pub mod error;

use std::str::FromStr;

use error::BlockchainError;
use subxt::{utils::AccountId32, OnlineClient, PolkadotConfig};
use sp_keyring::AccountKeyring;
use codec::Decode;

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
}