const instruction = new TransactionInstruction({
  keys: [{ pubkey: greetedPubkey, isSigner: false, isWritable: true }],
  programId,
  data: Buffer.alloc(0), // All instructions are hellos
});
await sendAndConfirmTransaction(
  connection,
  new Transaction().add(instruction),
  [payer]
);
