echo 'prepare phase1'
snarkjs powersoftau new bn128 12 pot12_0000.ptau -v
echo 'contribute phase1 first'
snarkjs powersoftau contribute pot12_0000.ptau pot12_0001.ptau --name="First contribution" -v
echo 'contribute phase1 second'
snarkjs powersoftau contribute pot12_0001.ptau pot12_0002.ptau --name="Second contribution" -v
echo 'apply a random beacon'
snarkjs powersoftau beacon pot12_0002.ptau pot12_beacon.ptau 0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f 10 -n="Final Beacon"
echo 'prepare phase2'
snarkjs powersoftau prepare phase2 pot12_beacon.ptau pot12_final.ptau -v
echo 'Verify the final ptau'
snarkjs powersoftau verify pot12_final.ptau
echo 'Remove staging files'
rm pot12_0000.ptau pot12_0001.ptau pot12_0002.ptau pot12_beacon.ptau