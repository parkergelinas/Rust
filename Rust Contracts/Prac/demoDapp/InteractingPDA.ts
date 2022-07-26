// Get Address

const [escrowPDA] = await.web3.PublicKey.findProgramAddress(
  [utf8.encode("escrow"), anchorWallet.publicKey.toBuffer(), toKey.toBuffer()],
  program.programId
);

// Get account data

const escrowAccount = await program.account.escrowAccount.fetch(escrowPDA);

// Sending a txn

const trans = await program.methods
  .createEscrow(new BN(30))
  .accounts({
    escrow: escrowPDA,
    from: anchorWallet.publicKey,
    to: toKey,
    systemProgram: web3.SystemProgram.programId,
  })
  .rpc();
