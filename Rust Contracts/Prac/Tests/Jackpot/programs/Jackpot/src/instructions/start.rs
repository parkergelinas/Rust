use anchor_lang::prelude::*;
use num_derive::*;
use num_traits::*;

pub mod jackpot {

    pub fn start(ctx: Context<Start>, player: Pubkey, bet_amount: u64, vendor_seed: i64) -> Result<()> {
        let jackpot: &mut Account<Jackpot> = &mut ctx.accounts.jackpot;

        jackpot.players = [];
        jackpot.vendor_seed = vendor_seed;
        jackpot.bump = *ctx.bumps.get(key: "jackpot").unwrap();
        jackpot.bet_amount = bet_amount;
    }
}

#[derive(Accounts)]
pub struct SetupGame<'info> {
    #[account(init, payer = player_one, space = Game::MAXIMUM_SIZE + 8)]
    pub game: Account<'info, Game>
    #[account(mut)]
    pub player_one: Signer<'info>,
    pub system_program: Program<'info, System>,
}