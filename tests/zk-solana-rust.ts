import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ZkSolanaRust } from "../target/types/zk_solana_rust";
import { parseProofToBytesArray, parseToBytesArray } from "../src/utils";
import jsonData from "../src/proof.js";

describe("zk-solana-rust", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ZkSolanaRust as Program<ZkSolanaRust>;
  
  const programProvider = program.provider as anchor.AnchorProvider;
    
  const gameKeypair = anchor.web3.Keypair.generate();
  const playerOne = programProvider.wallet;

  it("Is initialized!", async () => {
    const parsed_proof =  parseProofToBytesArray(JSON.stringify(jsonData.proof));
    // 创建 VerifyData 对象
    const verifyData = {
      publicSignals: parseToBytesArray(jsonData.publicSignals), // 填充这个数组
      proofA: parsed_proof.proofA,
      proofB: parsed_proof.proofB,
      proofC: parsed_proof.proofC,
    };

    // Add your test here.
    const tx = await program.methods.verify(verifyData).accounts({
      game: gameKeypair.publicKey,
      player1Host: playerOne.publicKey,
    }).signers([gameKeypair]).rpc();
    console.log("Your transaction signature", tx);
  });
});