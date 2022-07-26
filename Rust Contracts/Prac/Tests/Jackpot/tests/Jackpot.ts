import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Jackpot } from "../target/types/jackpot";

describe("Jackpot", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Jackpot as Program<Jackpot>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});