use anchor_lang::prelude::*;

declare_id!("6TxhQdES2UUnXBophnvcAWYV7jRVrrSq9ypyhCZn4q14");

#[program]
pub mod mint_machine {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
