import { PublicKey } from "@solana/web3.js";
import { BN } from "@project-serum/anchor";
import { CLOCKWORK_THREAD_PROGRAM_ID, THREAD } from "./constants";

export const getThreadPDA = async (authority: PublicKey, id: string) => {
  return PublicKey.findProgramAddress(
    [Buffer.from(THREAD), authority.toBuffer(), Buffer.from(id)],
    CLOCKWORK_THREAD_PROGRAM_ID
  );
};
