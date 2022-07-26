const network = "http://127.0.0.1:8899";
const connection = new Connection(network, "processed");
const provider = new AnchorProvider(connection, anchorWallet, {
  preflightCommitment: "processed",
});
const program = new Program(idl, idl.metadata.address, provider);
