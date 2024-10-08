echo "generate r1cs" 
circom ../circuits/shuffle.circom --r1cs
echo "create new zkey" 
snarkjs groth16 setup shuffle.r1cs pot12_final.ptau shuffle_0000.zkey
echo "some random text" 
snarkjs zkey contribute shuffle_0000.zkey shuffle_0001.zkey --name="1st Contributor Name" -v -e=abcdefgh
echo "another random text" 
snarkjs zkey contribute shuffle_0001.zkey shuffle_0002.zkey --name="Second contribution Name" -v -e=abcdefgh
echo "phase 3"
snarkjs zkey export bellman shuffle_0002.zkey  challenge_phase2_0003
snarkjs zkey bellman contribute bn128 challenge_phase2_0003 response_phase2_0003 -e="some random text"
snarkjs zkey import bellman shuffle_0002.zkey response_phase2_0003 shuffle.zkey -n="Third contribution name"
echo "verify zkey"
snarkjs zkey verify shuffle.r1cs pot12_final.ptau shuffle.zkey
echo 'Remove staging files'
rm shuffle_0000.zkey shuffle_0001.zkey shuffle_0002.zkey challenge_phase2_0003 response_phase2_0003