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