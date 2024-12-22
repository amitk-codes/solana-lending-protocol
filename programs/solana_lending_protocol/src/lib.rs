use anchor_lang::prelude::*;

declare_id!("8QgwtewNmdfZJiEKfiYz5Zzy5ZSMhSNk8r4XdeP9sNN1");

#[program]
pub mod solana_lending_protocol {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
