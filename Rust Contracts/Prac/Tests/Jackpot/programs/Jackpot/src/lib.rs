use anchor_lang::prelude::*;
use num_derive::*;
use num_traits::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod jackpot {
    use super::*;
    use anchor_lang::solana_program::{program::invoke, system_instruction::transfer};

    pub fn deposit(ctx: Context<Deposit>, player: Pubkey, bet_amount: u64, vendor_seed: i64) -> Result<()> {
        let jackpot = &mut ctx.accounts.jackpot_account;

        jackpot.players = [ctx.accounts.vendor.key(), player];
        jackpot.vendor_seed = vendor_seed;
        jackpot.bump = *ctx.bumps.get("jackpot").unwrap();
        jackpot.bet_amount = bet_amount;

        invoke(
            instruction:&transfer(
                from_pubkey: ctx.accounts.vendor.to_account_info().key,
                to_pubkey: jackpot.to_account_info().key,
                lamports: jackpot.bet_amount
            ),
            account_info: &[
                ctx.accounts.vendor.to_account_info(),
                jackpot.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?,

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, tile: Tile) -> Result<()> {
        instructions::play::play(ctx, tile)
    }
}
