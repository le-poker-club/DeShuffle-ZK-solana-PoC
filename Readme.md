## 1. Generate [Shuffle](circuits/shuffle.circom) ZKP Verifier
### 1. Prepare circuits
check the circuits folder

### 2. Prepare verification key
1. Generate ptau file
  - by CLI：`cd helpers && sh ptau.sh`
  - Download from internet：https://storage.googleapis.com/zkevm/ptau/powersOfTau28_hez_final_12.ptau
2. Generate zkey
  - by CLI：`cd helpers && sh zkey.sh`
3. Use zkey to generate verification key
```shell
# use zkey generated above
snarkjs zkey export verificationkey shuffle.zkey verification_key.json
```

### 3. Prepare rust verifier
Use [groth16-solana](https://github.com/Lightprotocol/groth16-solana) generate verifier
Copy the script to helpers folder，run `cd helpers && node parse_vk_to_rust.js verification_key.json`
export verifier file to helpers/verifying_key.rs 

## 2. Sonala test
Run `anchor test` to simulate solana onchain verification test

## 3. Workflow
![deshuffle](./deshuffle-workflow.png)