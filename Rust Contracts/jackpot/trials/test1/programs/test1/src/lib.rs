use anchor_lang::prelude::*;
use num_derive::*;
use num_traits::*;
use fair::games::wheel::*;


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod jackpot {
    use super::*;
    use anchor_lang::solana_program::{program::invoke, system_instruction::transfer};

    pub fn setup(ctx: Context<Initialize>, player: Pubkey, bet_amount: u64, vendor_seed: i64) -> Result<()> {
        let jackpot = &mut ctx.accounts.jackpot;

        jackpot.players = [ctx.accounts.vendors.key(), player];
        jackpot.vendor_seed = vendor_seed;
        jackpot.bump = *ctx.bumps.get("jackpot").unwrap();
        jackpot.bet_amount = bet_amount;

        invoke(
            &transfer(
                ctx.accounts.vendor.to_account_info().key,
                jackpot.to_account_info().key,
                jackpot.bet_amount,
            ),
            &[
                ctx.accounts.vendor.to_account_info(),
                jackpot.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        Ok(())
    }

    pub fn play(ctx: Context<Play>, player_choice: u8, player_seed: i64) -> Result<()> {
        let jackpot = &mut ctx.accounts.jackpot;
        let player_seed = player_seed;


    }

    pub fn delete(_ctx: Context<Delete>, player: Pubkey) -> Result<()> {
        Ok(())
    }
}


#[derive(Accounts)]
#[instruction(player: Pubkey, bet_amount: u64, vendor_seed: i64)]
pub struct Setup<'info> {
    #[account(
        init,
        payer = vendor,
        space = CoinFlip::LEN,
        seeds = [b"jackpot", vendor.key().as_ref(), player.as_ref()], bump
    )]
    pub jackpot: Account<'info, CoinFlip>,
    #[account(mut)]
    pub vendor: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Play<'info> {
    #[account(
        mut,
        seeds = [b"jackpot", vendor.key().as_ref(), player.key().as_ref()], bump
    )]
    pub jackpot: Account<'info, Jackpot>,
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut)]
    // Check
    pub vendor : AccountInfo<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(player: Pubkey)]
pub struct Delete<'info> {
    #[account(
        mut,
        close = vendor,
        seeds = [b"jackpot", vendor.key().as_ref(), player.key().as_ref()], bump
    )]
    pub jackpot: Account<'info, Jackpot>,
    #[account(mut)]
    pub vendor: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct Jackpot {
    players: [Pubkey; 99],
    vendor_seed: i64,
    state: JackpotState,
    bet_amount: u64,
    bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum JackpotState {
    Active,
    Finished { winner : Pubkey },
}

impl Default for JackpotState {
    fn default() -> Self {
        Self::Active
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, FromPrimitive, ToPrimitive, Copy, Clone, PartialEq, Eq)]
pub enum Tile {

}

impl Jackpot {
    const LEN: usize = 64 + 8 + 33 + 8 + 8 + 8;

    fn roll_tile(&self, roll_number: i64) -> Tile {
        if roll_number == 0 {

        }
    }

    fn roll(&self, player_seed: i64) -> Tile {
        let clock: Clock = Clock::get().unwrap();
        let roll_number: i64 = (self.vendor_seed + player_seed + clock.unix_timestamp) % 2;

        self.roll_tile(roll_number)
    }
}