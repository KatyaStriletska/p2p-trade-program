use anchor_lang::prelude::*;
use anchor_spl::token::{ Token, TokenAccount, Mint, TransferChecked, transfer_checked};
use anchor_spl::associated_token::AssociatedToken;

use crate::states::Escrow;
use crate::states::EscrowError;
use crate::states::EscrowStage;

#[derive(Accounts)]
#[instruction(params: CreateParams)]
pub struct CreateTrade<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 + Escrow::LEN, 
        seeds = [b"trade", creator.key().as_ref(), params.trade_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        init,
        payer = creator,
        associated_token::mint = token_for_sale,
        associated_token::authority = escrow
    )]
    pub escrow_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        mut,
        constraint = creator_ata_for_sale.owner == creator.key(),
        constraint = creator_ata_for_sale.minsot == token_for_sale.key()
    )]
    pub creator_ata_for_sale: Account<'info, TokenAccount>,

    pub token_for_sale: Account<'info, Mint>,

    #[account(
        // constraint = received_token_mint_account.key() == TEST_USDC_MINT_ADDRESS.parse::<Pubkey>().unwrap() @ EscrowError::InvalidReceiveMint
    )]
    pub received_token_mint_account: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>, 
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct CreateParams {
    pub trade_id: u64,
    pub trade_amount: u64,
    pub expected_amount: u64, 
    pub recipient: Option<Pubkey>,
}

pub fn create_trade(
    ctx: Context<CreateTrade>,
    params: CreateParams,
) -> Result<()> {
    require_gt!(params.trade_amount, 0, EscrowError::ZeroValue);
    require_gt!(params.expected_amount, 0, EscrowError::ZeroValue);
    //require_keys_eq!(params.received_token_mint,  testUSDC);
   
    let decimals = ctx.accounts.token_for_sale.decimals;

    transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.creator_ata_for_sale.to_account_info(),
                to: ctx.accounts.escrow_vault.to_account_info(),
                authority: ctx.accounts.creator.to_account_info(),
                mint: ctx.accounts.token_for_sale.to_account_info(),
            },
        ),
        params.trade_amount,
        decimals,
    )?;

    let escrow = &mut ctx.accounts.escrow;
    escrow.owner = ctx.accounts.creator.key();
    escrow.recipient = params.recipient;
    escrow.trade_token_mint = ctx.accounts.token_for_sale.key();
    escrow.trade_amount = params.trade_amount;
    escrow.received_token_mint = ctx.accounts.received_token_mint_account.key(); // testUSDC
    escrow.expected_amount = params.expected_amount;
    escrow.escrow_vault = ctx.accounts.escrow_vault.key();
    escrow.stage = EscrowStage::ReadyExchange;
    escrow.trade_id = params.trade_id;
    escrow.state_bump = ctx.bumps.escrow;


    Ok(())
}