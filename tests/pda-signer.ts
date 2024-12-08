import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PdaSigner } from "../target/types/pda_signer";
import { MyCpi } from "../target/types/my_cpi";
import { assert } from "chai";

describe("pda-signer", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.AnchorProvider.env();

  const pdaSignerProgram = anchor.workspace.PdaSigner as Program<PdaSigner>;
  const myCpiProgram = anchor.workspace.MyCpi as Program<MyCpi>;

  it("Is initialized!", async () => {
    const [pdaOne, pdaOneBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("pda_one")],
      pdaSignerProgram.programId
    );
    const [pdaTwo, pdaTwoBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("pda_two")],
      pdaSignerProgram.programId
    );
    const [counter, counterBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("pda_three")],
      pdaSignerProgram.programId
    );
    const [signerPdaOne, signerPdaOneBump] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("pda_one")],
        myCpiProgram.programId
      );
    const [signerPdaTwo, signerPdaTwoBump] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("pda_two")],
        myCpiProgram.programId
      );
    console.log("pdaOne", pdaOne.toBase58());
    console.log("pdaTwo", pdaTwo.toBase58());
    console.log("counter", counter.toBase58());
    console.log("signerPdaOne", signerPdaOne.toBase58());
    console.log("signerPdaTwo", signerPdaTwo.toBase58());
    // Add your test here.
    const tx = await pdaSignerProgram.methods
      .initialize()
      .accounts({
        signer: provider.publicKey,
        pdaOne,
        pdaTwo,
        counter,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    console.log("Your transaction signature", tx);
    const counterAccount = await pdaSignerProgram.account.counter.fetch(
      counter
    );
    console.log(counterAccount);
    assert.equal(counterAccount.count, 0);

    const tx2 = await myCpiProgram.methods
      .initialize()
      .accounts({
        signer: provider.publicKey,
        pdaSigner: pdaSignerProgram.programId,
        pdaOne,
        pdaTwo,
        signerPdaOne,
        signerPdaTwo,
        counter,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    console.log("Your transaction signature", tx2);
  });
});
