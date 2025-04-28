use anchor_lang::prelude::*;
pub mod instructions;
pub mod states;

use instructions::*;
declare_id!("EAZ9GhM4TTfSrQAPB9VRdzurU6rTRMM73ZupUDA6geeC");
#[program]
pub mod p2p_trade_program {
    use super::*;

    pub fn create_trade(
        ctx: Context<CreateTrade>,
        params: CreateParams,
    ) -> Result<()> {
        instructions::create::create_trade(ctx, params)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
