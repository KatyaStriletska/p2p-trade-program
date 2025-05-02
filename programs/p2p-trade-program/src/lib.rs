use anchor_lang::prelude::*;
pub mod instructions;
pub mod states;

use instructions::*;
declare_id!("F6UDhDHeqvTrQRtaQPkTnwXf3NVCSYZniiMQbAMA1Q1T");
#[program]
pub mod p2p_trade_program {
    use super::*;

    pub fn create_trade(ctx: Context<CreateTrade>, params: CreateParams) -> Result<()> {
        instructions::create::create_trade(ctx, params)
    }
    pub fn exchange(ctx: Context<Exchange>, trade_id: u64) -> Result<()> {
        instructions::exchange::exchange(ctx, trade_id)
    }
    pub fn cancel(ctx: Context<Cancel>, trade_id: u64) -> Result<()> {
        instructions::cancel::cancel(ctx, trade_id)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
