import * as anchor from "@project-serum/anchor";
import { Program, BN } from "@project-serum/anchor";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  SystemProgram,
} from "@solana/web3.js";
import autopayIdl from "../target/idl/autopay.json";
import { AUTOPAY_PROGRAM_ID, CLOCKWORK_THREAD_PROGRAM_ID } from "../utils/constants";
import { getThreadPDA, airdrop } from "../utils";
import schedulerIdl from "../deps/cronos/target/idl/cronos_scheduler.json";

describe("autopay", () => {
  // Configure the client to use the local cluster.
  const program = new Program(autopayIdl as anchor.Idl, AUTOPAY_PROGRAM_ID, anchor.Provider.env())
  const { wallet } = program.provider;

  it("Is initialized!", async () => {
    const caller = Keypair.generate();
    const receiver = Keypair.generate();

    await airdrop(
      caller.publicKey,
      10 * LAMPORTS_PER_SOL,
      program.provider.connection
    );

    const threadId = "thread";
    const amount = new BN(0.5 * LAMPORTS_PER_SOL);
    const [thread] = await getThreadPDA(caller.publicKey, threadId);

    await program.methods
      .createThread(threadId, amount, { immediate: {} }, null)
      .accounts({
        authority: caller.publicKey,
        payer: caller.publicKey,
        receiver: receiver.publicKey,
        thread,
        systemProgram: SystemProgram.programId,
        threadProgram: CLOCKWORK_THREAD_PROGRAM_ID,
      })
      .signers([caller])
      .rpc();
  });
});
