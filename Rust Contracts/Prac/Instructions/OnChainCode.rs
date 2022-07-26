// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the program was laoded into
    accounts: &[AccountInfo], // The account to say hello to
    _instruction_data: &[u8], // ---
) -> ProgramResult {
    msg!("Hello World Rust program entrypoint");
}