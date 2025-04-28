use anchor_lang::prelude::*;

declare_id!("zhSMy6zXkvfyrajwhpq6bnEnioXuuXHiWjywkd2TYUp");

#[program]
pub mod p2p_trade_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
