use anchor_lang::prelude::*;
use anchor_spl::token::{ 
    Token, TokenAccount, 
    Mint, 
    TransferChecked, transfer_checked, 
    CloseAccount, close_account};
use anchor_spl::associated_token::AssociatedToken;

use crate::states::Escrow;
use crate::states::EscrowError;
use crate::states::EscrowStage;

#[derive(Accounts)]
#[instruction(trade_id: u64)]
pub struct Cancel<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = trade_token_mint,
        associated_token::authority = escrow_state)]
    pub escrow_vault: Account<'info, TokenAccount>,

    #[account(
        mut, 
        has_one=owner, 
        has_one=escrow_vault,
        seeds=[b"trade", owner.key().as_ref(), trade_id.to_le_bytes().as_ref()],
        bump = escrow_state.state_bump,
        constraint = escrow_state.stage == EscrowStage::ReadyExchange @ EscrowError::InvalidStage,
        close=owner,
    )]
    pub escrow_state: Account<'info, Escrow>,

    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = trade_token_mint,
        associated_token::authority = owner 
    )]
    pub owner_trade_token_ata: Account<'info, TokenAccount>,

    #[account(
        address = escrow_state.trade_token_mint.key() @ EscrowError::InvalidMint
    )]
    pub trade_token_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn cancel(
    ctx: Context<Cancel>, trade_id: u64
) -> Result<()> {
    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.escrow_vault.to_account_info(),
                to: ctx.accounts.owner_trade_token_ata.to_account_info(),
                authority: ctx.accounts.escrow_state.to_account_info(),
                mint: ctx.accounts.trade_token_mint.to_account_info(),
            },
            &[&[
                b"trade",
                ctx.accounts.owner.key().as_ref(),
                &trade_id.to_le_bytes(),
                &[ctx.accounts.escrow_state.state_bump],
            ]],
        ), 
        ctx.accounts.escrow_vault.amount,
        ctx.accounts.trade_token_mint.decimals,
    )?;

    close_account(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            CloseAccount {
                account: ctx.accounts.escrow_vault.to_account_info(),
                destination: ctx.accounts.owner.to_account_info(),
                authority: ctx.accounts.escrow_state.to_account_info(),
            },
            &[&[
                b"trade",
                ctx.accounts.owner.key().as_ref(),
                &trade_id.to_le_bytes(),
                &[ctx.accounts.escrow_state.state_bump],
            ]],
        
        ),
    )?;

    Ok(())
}