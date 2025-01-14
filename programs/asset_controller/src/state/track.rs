use anchor_lang::prelude::*;
use policy_engine::Transfer;

use crate::AssetControllerErrors;

pub const MAX_TRANSFER_HISTORY: usize = 25;

#[account()]
#[derive(InitSpace)]
pub struct TrackerAccount {
    pub version: u8,
    // corresponding asset mint
    pub asset_mint: Pubkey,
    // owner of the policy account
    pub owner: Pubkey,
    // transfer amounts
    #[max_len(MAX_TRANSFER_HISTORY)] // 25 is the max number of transfers we want to store
    pub transfers: Vec<Transfer>,
}

impl TrackerAccount {
    pub const VERSION: u8 = 1;
    pub fn new(&mut self, asset_mint: Pubkey, owner: Pubkey) {
        self.version = Self::VERSION;
        self.asset_mint = asset_mint;
        self.owner = owner;
    }
    /// for all timestamps, if timestamp is older than timestamp - max_timeframe. remove it,
    pub fn update_transfer_history(
        &mut self,
        amount: u64,
        timestamp: i64,
        max_timeframe: i64,
    ) -> Result<()> {
        let min_timestamp = timestamp - max_timeframe;
        self.transfers
            .retain(|transfer| transfer.timestamp >= min_timestamp);
        self.transfers.push(Transfer { amount, timestamp });
        // return error if the transfer history is too large
        if self.transfers.len() > MAX_TRANSFER_HISTORY {
            return Err(AssetControllerErrors::TransferHistoryFull.into());
        }
        Ok(())
    }
}
