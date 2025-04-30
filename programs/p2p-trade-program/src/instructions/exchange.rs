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
pub struct Exchange<'info>{
    #[account(mut)]
    pub buyer: Signer<'info>,
    
    #[account(mut)]
    /// CHECK: This is the original creator of the escrow. We check ownership against the escrow account using has_one constraint. Used only as ATA authority.
    pub owner: SystemAccount<'info>,

    #[account( mut,
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
    )]
    pub escrow_state: Account<'info, Escrow>,

    #[account(
        init_if_needed, 
        payer = buyer,
        associated_token::mint = trade_token_mint,
        associated_token::authority = buyer
    )]
    pub buyer_receives_sale_token_ata: Account<'info, TokenAccount>,

    #[account(
        init_if_needed, 
        payer = buyer, 
        associated_token::mint = received_token_mint, 
        associated_token::authority = buyer 
    )]
    pub buyer_pays_with_usdc_ata: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = received_token_mint,
        associated_token::authority = owner

    )]
    pub creator_ata_for_usdc: Account<'info, TokenAccount>,

    //spl-token
    #[account(
        address = escrow_state.trade_token_mint @ EscrowError::InvalidMint
    )]
    pub trade_token_mint: Account<'info, Mint>,
    //uscd
    #[account(
        address = escrow_state.received_token_mint @ EscrowError::InvalidReceiveMint
    )]
    pub received_token_mint: Account<'info, Mint>,


    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,

}

pub fn exchange(ctx: Context<Exchange>, trade_id: u64) -> Result<()>{

    transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.buyer_pays_with_usdc_ata.to_account_info(),
                to: ctx.accounts.creator_ata_for_usdc.to_account_info(),
                authority: ctx.accounts.buyer.to_account_info(),
                mint: ctx.accounts.received_token_mint.to_account_info(),
            },
        ),
        ctx.accounts.escrow_state.expected_amount,
        ctx.accounts.received_token_mint.decimals,
    )?;

    // transfer project tokens
    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.escrow_vault.to_account_info(),
                to: ctx.accounts.buyer_receives_sale_token_ata.to_account_info(),
                authority: ctx.accounts.escrow_state.to_account_info(),
                mint: ctx.accounts.trade_token_mint.to_account_info(),
            },
            &[&[
                b"trade",
                ctx.accounts.owner.key().as_ref(),
                trade_id.to_le_bytes().as_ref(),
                &[ctx.accounts.escrow_state.state_bump],
            ]],
        ),
        ctx.accounts.escrow_state.trade_amount,
        ctx.accounts.trade_token_mint.decimals,
    )?;

    ctx.accounts.escrow_state.stage = EscrowStage::Exchanged;

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
                trade_id.to_le_bytes().as_ref(),
                &[ctx.accounts.escrow_state.state_bump],
            ]],
        ),
    )?;


    Ok(())
}