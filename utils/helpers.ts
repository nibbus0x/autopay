import * as anchor from "@project-serum/anchor";

export enum Trigger {}

export const airdrop = async (
  publicKey: anchor.web3.PublicKey,
  lamports: number,
  connection: anchor.web3.Connection
) => {
  const { blockhash, lastValidBlockHeight } =
    await connection.getLatestBlockhash();

  await connection.confirmTransaction(
    {
      signature: await connection.requestAirdrop(publicKey, lamports),
      blockhash,
      lastValidBlockHeight,
    },
    "confirmed"
  );
};
