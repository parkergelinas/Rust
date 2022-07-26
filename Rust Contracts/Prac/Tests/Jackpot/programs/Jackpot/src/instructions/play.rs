use anchor_lang::prelude::*;
use num_derive::*;
use num_traits::*;

pub fn play(ctx: Context<Play>, player_choice: u8, player_seed: i64) -> Result<()> {
    let jackpot = &mut ctx.accounts.jackpot;
    let player_seed = player_seed;


    let player_side = if player_choice == 0 {

    };

    invoke(
        &transfer(
            ctx.accounts.player.to_account_info().key,
            jackpot.to_account_info().key,
            jackpot.bet_amount,
        ),
        &[
            ctx.accounts.player.to_account_info(),
            jackpot.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    let total_bet = jackpot.bet_amount * 2;

    let winner = jackpot.play(player_seed, player_side);

    **jackpot.to_account_info().try_borrow_mut_lamports()? -= total_bet;

    if winner == *ctx.accounts.vendor.key {
        **ctx.accounts.vendor.try_borrow_mut_lamports()? += total_bet;
    } else {
        **ctx.accounts.player.to_account_info().try_borrow_mut_lamports()? += total_bet;
    }

    Ok(())
}