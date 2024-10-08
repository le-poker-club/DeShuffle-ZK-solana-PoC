import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LePokerVdfSolana } from "../target/types/le_poker_vdf_solana";
import { expect } from "chai";
import { Connection } from '@solana/web3.js';
import { generateZKP, hashPoseidon } from "../js-sdk/utilities";
import { createHash } from 'crypto';

const X = "2065743662758022685238145802486066419409038405748";
const GAME_ID = "6jvHTtq:2";
const TIME = 417337200;
const PK = 92470579;
const Y = new anchor.BN(91525589);
// const Y = 28520076;
const PI = 88035170;
const CHALLENGE = "7230909640199066306989126415855635493";

describe("le-poker-vdf-solana", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.LePokerVdfSolana as Program<LePokerVdfSolana>;
  const gameKeyPair = anchor.web3.Keypair.generate();
  console.log("Game key pair", gameKeyPair.publicKey.toString());

  it("Game start", async () => {
    const createTime = Math.floor(new Date().getTime() / 1000);
    const tx = await program.methods.gameStart(
      GAME_ID,
      new anchor.BN(PK),
      new anchor.BN(TIME),
      X,
      ["player1", "player2"],
      new anchor.BN(createTime),
    )
      .accounts({
        game: gameKeyPair.publicKey
      })
      .signers([gameKeyPair])
      .rpc();
    console.log("Your transaction signature", tx);

    await (new Promise((resolve) => setTimeout(resolve, 3000)));
    const connection = new Connection('http://localhost:8899', 'confirmed');
    const txDetails = await connection.getTransaction(tx, {
      commitment: 'confirmed',
    });
    console.log('Gas Fee:', (txDetails.meta.preBalances[0] - txDetails.meta.postBalances[0]) / 1e9);
    console.log('Computed Units:', txDetails.meta.computeUnitsConsumed);

    let gameState = await program.account.game.fetch(gameKeyPair.publicKey);
    expect(gameState.gameId).to.equal(GAME_ID);
    expect(gameState.state).to.eql({ ongoing: {} });
  });

  it("Game end", async () => {
    const additionalComputeBudgetInstruction =
      anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
        units: 10_000_000,
      });

    const bigNumBytes = Y.toArrayLike(Buffer, 'be');

    const expectedHash = createHash('sha256').update(bigNumBytes).digest();
    const expectedHashArray = Array.from(expectedHash);

    const tx = await program.methods.gameEnd(
      expectedHashArray,
      new anchor.BN(PI),
      CHALLENGE,
    )
      .accounts({
        game: gameKeyPair.publicKey
      })
      .preInstructions([additionalComputeBudgetInstruction])
      .rpc();
    console.log("Your transaction signature", tx);

    await (new Promise((resolve) => setTimeout(resolve, 3000)));
    const connection = new Connection('http://localhost:8899', 'confirmed');
    const txDetails = await connection.getTransaction(tx, {
      commitment: 'confirmed',
    });
    console.log('Gas Fee:', (txDetails.meta.preBalances[0] - txDetails.meta.postBalances[0]) / 1e9);
    console.log('Computed Units:', txDetails.meta.computeUnitsConsumed);

    let gameState = await program.account.game.fetch(gameKeyPair.publicKey);
    expect(gameState.gameId).to.equal(GAME_ID);
    expect(gameState.state).to.eql({ ended: {} });
  });

  it("Game update cards", async () => {
    const additionalComputeBudgetInstruction =
      anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
        units: 10_000_000,
      });

    const cards = [30, 51, 1, 19, 25, 27, 32, 6, 2, 13, 22,
      28, 44, 18, 15, 37, 23, 10, 52, 36, 46, 9,
      11, 21, 16, 8, 50, 24, 48, 12, 4, 7, 49,
      33, 42, 5, 29, 45, 38, 26, 40, 14, 47, 20,
      17, 31, 35, 43, 39, 3, 41, 34];
    const zkp = await generateZKP(28520076, cards);

    const tx = await program.methods.gameCardsVerify(
      zkp.seedHashArray,
      zkp.proofA,
      zkp.proofB,
      zkp.proofC,
      cards,
    )
      .accounts({
        game: gameKeyPair.publicKey
      })
      .preInstructions([additionalComputeBudgetInstruction])
      .rpc();
    console.log("Your transaction signature", tx);

    await (new Promise((resolve) => setTimeout(resolve, 3000)));
    const connection = new Connection('http://localhost:8899', 'confirmed');
    const txDetails = await connection.getTransaction(tx, {
      commitment: 'confirmed',
    });
    console.log('Gas Fee:', (txDetails.meta.preBalances[0] - txDetails.meta.postBalances[0]) / 1e9);
    console.log('Computed Units:', txDetails.meta.computeUnitsConsumed);

    let gameState = await program.account.game.fetch(gameKeyPair.publicKey);
    expect(gameState.gameId).to.equal(GAME_ID);
    expect(gameState.state).to.eql({ ended: {} });
  });
});
