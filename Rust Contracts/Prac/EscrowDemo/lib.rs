use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("");

#[program]
pub mod demo_pda {
    use super::*;

    pub fn create_escrow(ctx: Context<CreateEscrow>, amount: u64) -> Result<()> {
        // Get Escrow Account
        let escrow = &mut ctx.accounts.escrow;

        // Set from
        escrow.from = ctx.accounts.from.key();
        // Set to
        escrow.to = ctx.accounts.to.key();
        // Set Amount
        escrow.amount = amount;

        Ok(())
    }
}

/// Create Escrow Context
#[derive(Accounts)]
pub struct CreateEscrow<'info>{
    //Escrow Account PDA
    #[account(
        init,
        // State account seed uses the string 'state' and the users key.
        // Note that we can only have 1 active transaction
        seeds = [b"escrow".as_ref(), from.key().as_ref(), to.key().as_ref()],
        bump,
        payer = from,
        space = size_of::<EscrowAccount>,
    )]
    pub escrow: Account<'info, EscrowAccount>,

    #[account(mut)]
    pub from: Signer<'info>,
    /// Check: Safe
    #[account(mut)]
    pub to: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

// Escrow Account Structure
#[account]
pub struct EscrowAccount {
    // From address
    pub from: Pubkey,

    // To address
    pub to: Pubkey,

    // Amount that is owed
    pub amount: u64,
}