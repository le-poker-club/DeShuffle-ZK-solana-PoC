echo 'prepare phase1'
snarkjs powersoftau new bn128 20 pot20_0000.ptau -v
echo 'contribute phase1 first'
snarkjs powersoftau contribute pot20_0000.ptau pot20_0001.ptau --name="First contribution" -v
echo 'contribute phase1 second'
snarkjs powersoftau contribute pot20_0001.ptau pot20_0002.ptau --name="Second contribution" -v
echo 'apply a random beacon'
snarkjs powersoftau beacon pot20_0002.ptau pot20_beacon.ptau 0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f 10 -n="Final Beacon"
echo 'prepare phase2'
snarkjs powersoftau prepare phase2 pot20_beacon.ptau pot20_final.ptau -v
echo 'Verify the final ptau'
snarkjs powersoftau verify pot20_final.ptau
echo 'Remove staging files'
rm pot20_0000.ptau pot20_0001.ptau pot20_0002.ptau pot20_beacon.ptau