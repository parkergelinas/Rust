use anchor_lang::prelude::*;
use num_derive::*;
use num_traits::*;


#[program]

#[account]
pub struct Game {
    players: [Pubkey; 2],          // (32 * 2)
    deposit: u8,                   // 1
    pot: [[Option<Sign>; 3]; 3],   // 9 * (1 + 1) = 18
    state: GameState,              // 32 + 1
}