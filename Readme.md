## 1. Generate [ShuffleEncrypt](circuits/shuffle_encrypt/shuffle_encrypt.circom) ZKP Verifier
### 1. Prepare circuits
check the circuits folder

### 2. Prepare verification key
1. Generate ptau file
  - by CLI：`cd helpers && sh ptau.sh`
  - Download from internet：https://p0x-labs.s3.amazonaws.com/zkShuffle/ptau.20 
2. Generate zkey
  - by CLI：`cd helpers && sh zkey.sh`
  - Download from internet：https://p0x-labs.s3.amazonaws.com/zkShuffle/encrypt.zkey 
3. Use zkey to generate verification key
```shell
# use zkey generated above
snarkjs zkey export verificationkey shuffle_encrypt.zkey verification_key.json
```

### 3. Prepare rust verifier
Use [groth16-solana](https://github.com/Lightprotocol/groth16-solana) generate verifier
Copy the script to helpers folder，run `cd helpers && node parse_vk_to_rust.js verification_key.json`
export verifier file to helpers/verifying_key.rs 

## 2. verifying_key.rs local test
test file locate in [verify_test.rs](programs/zk-shuffle-solana-rust/src/verify_test.rs)
Run `cargo test` for local test

## 3. Sonala test
Run `anchor test` to simulate solana onchain verification test
