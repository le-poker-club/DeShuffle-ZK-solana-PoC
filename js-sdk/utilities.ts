import { buildPoseidon } from "circomlibjs";
import { resolve } from "path";
import * as snarkjs from "snarkjs";
import { utils } from "ffjavascript";
const { unstringifyBigInts, leInt2Buff } = utils;

export function bigintToArray(num: string, bits = 32, size = 8) {
  const mask = BigInt(2 ** bits - 1);
  const arr = new Array(size).fill(0);

  for (let i = 0; i < size; i++) {
    arr[i] = (BigInt(num) >> BigInt(32 * i)) & mask;
  }

  return arr;
}

function parseProofToBytesArray(mydata: any) {
  for (var i in mydata) {
    if (i == "pi_a" || i == "pi_c") {
      for (var j in mydata[i]) {
        mydata[i][j] = Array.from(
          leInt2Buff(unstringifyBigInts(mydata[i][j]), 32),
        ).reverse();
      }
    } else if (i == "pi_b") {
      for (var j in mydata[i]) {
        for (var z in mydata[i][j]) {
          mydata[i][j][z] = Array.from(
            leInt2Buff(unstringifyBigInts(mydata[i][j][z]), 32),
          );
        }
      }
    }
  }

  return {
    proofA: [mydata.pi_a[0], mydata.pi_a[1]].flat(),
    proofB: [
      mydata.pi_b[0].flat().reverse(),
      mydata.pi_b[1].flat().reverse(),
    ].flat(),
    proofC: [mydata.pi_c[0], mydata.pi_c[1]].flat(),
  };
}

function parseToBytesArray(publicSignals: Array<string>) {

  var publicInputsBytes = new Array<Array<number>>();
  for (var i in publicSignals) {
    let ref: Array<number> = Array.from([
      ...leInt2Buff(unstringifyBigInts(publicSignals[i]), 32),
    ]).reverse();
    publicInputsBytes.push(ref);

  }

  return publicInputsBytes
}

export async function hashPoseidon(input: number) {
  const poseidonHash = await buildPoseidon();

  const combinedHash = poseidonHash([input]);
  const combinedHashString = poseidonHash.F.toString(combinedHash);
  
  return combinedHashString;
}

export async function generateZKP(
  seed: number,
  numbers: number[],
) {
  // generate proof
  const wasmFile = resolve(__dirname, "../circuits/shuffle.wasm");
  const zkeyFile = resolve(__dirname, "../circuits/shuffle.zkey");

  const seedHash = await hashPoseidon(seed);

  const proof = await snarkjs.groth16.fullProve({
    seed, seedHash, numbers
  }, wasmFile, zkeyFile);

  const proofData = parseProofToBytesArray(proof.proof);
  const publicSingalsData = parseToBytesArray(proof.publicSignals);

  return { ...proofData, seedHash, seedHashArray: publicSingalsData[0] };
}
