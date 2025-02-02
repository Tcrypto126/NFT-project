use anchor_lang::prelude::*;

declare_id!("BTMq1LQVA8sDLgjngNkvE4SSNjLHUabX3rHznez5VDeJ");

#[program]
pub mod nft_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
