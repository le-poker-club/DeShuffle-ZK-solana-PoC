const { shuffleEncryptV2Plaintext } = require("@zk-shuffle/proof/src/shuffle/plaintext");
const { generateShuffleEncryptV2Proof } = require("@zk-shuffle/proof/src/shuffle/proof");
const {
  initDeck,
  keyGen,
  sampleFieldElements,
  samplePermutation,
  compressDeck,
  recoverDeck,
} = require("@zk-shuffle/proof/src/shuffle/utilities");
const { resolve } = require("path");
const { readFileSync } = require("fs");
const buildBabyjub = require("circomlibjs").buildBabyjub;
const snarkjs = require("snarkjs");

const { buildBls12381, utils } = require("ffjavascript");

const numBits = BigInt(251);
const numCards = BigInt(52);
const numProfiling = 3;

async function main() {
  const babyjub = await buildBabyjub();
  const keysAlice = keyGen(babyjub, numBits);
  const pk = keysAlice.pk;
  const pkString = [babyjub.F.toString(keysAlice.pk[0]), babyjub.F.toString(keysAlice.pk[1])];

  const encryptWasmFile = resolve(__dirname, "./helpers/shuffle_encrypt.wasm");
  const encryptZkeyFile = resolve(__dirname, "./helpers/shuffle_encrypt.zkey");
  const encryptVkey = await snarkjs.zKey.exportVerificationKey(
    new Uint8Array(Buffer.from(readFileSync(encryptZkeyFile))),
  );

  // Initializes deck.
  const initializedDeck: bigint[] = initDeck(babyjub, Number(numCards));
  const compressedDeck = compressDeck(initializedDeck);
  const deck: {
    X0: bigint[];
    X1: bigint[];
    selector: bigint[];
  } = {
    X0: compressedDeck.X0,
    X1: compressedDeck.X1,
    selector: compressedDeck.selector,
  };

  const A = samplePermutation(Number(numCards));
  const R = sampleFieldElements(babyjub, numBits, numCards);
  const deckDelta = recoverDeck(babyjub, deck.X0, deck.X1);
  const plaintext_output = shuffleEncryptV2Plaintext(
    babyjub,
    Number(numCards),
    A,
    R,
    pk,
    deck.X0,
    deck.X1,
    deckDelta.Delta0,
    deckDelta.Delta1,
    deck.selector,
  );
  console.log(
    pkString,
    A,
    R,
    deck.X0,
    deck.X1,
    deckDelta.Delta0,
    deckDelta.Delta1,
    deck.selector,
    plaintext_output.X0,
    plaintext_output.X1,
    plaintext_output.delta0,
    plaintext_output.delta1,
    plaintext_output.selector,
  );
  const shuffleEncryptOutput = await generateShuffleEncryptV2Proof(
    pkString,
    A,
    R,
    deck.X0,
    deck.X1,
    deckDelta.Delta0,
    deckDelta.Delta1,
    deck.selector,
    plaintext_output.X0,
    plaintext_output.X1,
    plaintext_output.delta0,
    plaintext_output.delta1,
    plaintext_output.selector,
    encryptWasmFile,
    encryptZkeyFile,
  );
  console.log(
    await snarkjs.groth16.verify(
      encryptVkey,
      shuffleEncryptOutput.publicSignals,
      shuffleEncryptOutput.proof,
    ),
  );
}

main();
