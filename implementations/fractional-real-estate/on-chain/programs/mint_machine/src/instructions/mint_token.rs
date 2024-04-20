use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct MintTokensArgs {
    pub amount: u64,
    pub to: Pubkey,
}

pub struct MintTokens<'info> {

}
