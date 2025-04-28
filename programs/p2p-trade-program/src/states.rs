use anchor_lang::prelude::*;

#[account]
pub struct Escrow {
    pub owner: Pubkey,
    pub recipient: Option<Pubkey>,
    pub trade_token_mint: Pubkey, // token, that is being traded
    pub trade_amount: u64, 
    pub received_token_mint: Pubkey, // token, that is being received testUSDC
    pub escrow_vault: Pubkey, // vault account for the trade PDA address
    pub stage: EscrowStage,
    pub trade_id: u64, //String,
    // for pda esrow_vault
    pub state_bump: u8,
    // pub vault_bump: u8,
    pub expected_amount: u64,
}
impl Escrow {
    pub const LEN: usize = 188;
}
#[derive(Clone, Copy, PartialEq, Eq, Debug, AnchorSerialize, AnchorDeserialize)]
#[repr(u8)]
pub enum EscrowStage {
    ReadyExchange = 1,
    Exchanged = 2,
    CancelTrade = 3,
}


pub const ESCROW_SIZE: usize = 8 + 32 + 32 + 32 + 8 + 8 + 1 + 1 + 8 + 8;

#[error_code]
pub enum EscrowError {
    #[msg("The trade amount is zero")]
    ZeroValue,
    // #[msg("The trade is already exchanged")]
    // AlreadyExchanged,
    // #[msg("The trade is not in the correct stage")]
    // InvalidStage,
    // #[msg("The trade is already cancelled")]
    // AlreadyCancelled,
    // #[msg("The trade is not in the correct stage")]
    // InvalidTradeId,
    // #[msg("The trade is not in the correct stage")]
    // InvalidTradeAmount,
}