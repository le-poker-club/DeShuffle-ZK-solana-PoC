use super::verifying_key::VERIFYINGKEY;
use ark_ff::bytes::{FromBytes, ToBytes};
use groth16_solana::groth16::Groth16Verifier;
use std::ops::Neg;

pub fn verify(
    proof_a: [u8; 64],
    proof_b: [u8; 128],
    proof_c: [u8; 64],
    y_hash: [u8; 32],
    public_signals: [u8; 52],
) -> bool {
    let proof_a_neg = handle_proof_a(&proof_a);

    let mut public_signals_slices = [[0u8; 32]; 53];
    y_hash.iter().enumerate().for_each(|(i, x)| {
        public_signals_slices[0][i] = *x;
    });
    for i in 1..53 {
        public_signals_slices[i][31] = public_signals[i - 1];
    }

    let mut verifier = Groth16Verifier::new(
        &proof_a_neg,
        &proof_b,
        &proof_c,
        &public_signals_slices,
        &VERIFYINGKEY,
    )
    .unwrap();

    verifier.verify().unwrap()
}

fn change_endianness(bytes: &[u8]) -> Vec<u8> {
    let mut vec = Vec::new();
    for b in bytes.chunks(32) {
        for byte in b.iter().rev() {
            vec.push(*byte);
        }
    }
    vec
}

fn handle_proof_a(proof_a: &[u8; 64]) -> [u8; 64] {
    type G1 = ark_ec::short_weierstrass_jacobian::GroupAffine<ark_bn254::g1::Parameters>;

    let proof_a_neg_g1: G1 = <G1 as FromBytes>::read(
        &*[&change_endianness(proof_a.as_slice())[..], &[0u8][..]].concat(),
    )
    .unwrap();

    let mut proof_a_neg = [0u8; 65];
    <G1 as ToBytes>::write(&proof_a_neg_g1.neg(), &mut proof_a_neg[..]).unwrap();
    let proof_a_neg = change_endianness(&proof_a_neg[..64]).try_into().unwrap();

    return proof_a_neg;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_success() {
        let result = verify(
            [
                37, 213, 170, 172, 206, 169, 161, 244, 134, 242, 213, 60, 225, 138, 62, 75, 210,
                201, 39, 33, 43, 58, 245, 160, 218, 157, 33, 134, 122, 16, 0, 163, 35, 68, 213, 2,
                230, 84, 17, 46, 157, 96, 247, 115, 23, 147, 239, 203, 67, 238, 112, 242, 164, 14,
                53, 123, 217, 191, 247, 62, 7, 195, 143, 184,
            ],
            [
                45, 132, 105, 13, 195, 73, 132, 131, 194, 219, 136, 181, 53, 173, 138, 17, 141,
                183, 188, 85, 103, 128, 182, 218, 160, 24, 60, 226, 147, 81, 150, 241, 12, 166,
                137, 121, 139, 226, 12, 210, 218, 113, 227, 246, 40, 0, 79, 95, 37, 184, 191, 8,
                54, 183, 15, 8, 208, 124, 179, 182, 110, 60, 157, 42, 45, 108, 114, 99, 244, 171,
                50, 130, 198, 55, 157, 87, 202, 138, 166, 101, 34, 14, 253, 240, 114, 202, 190,
                163, 232, 80, 138, 81, 85, 2, 183, 29, 36, 204, 10, 8, 12, 129, 42, 15, 215, 67,
                215, 146, 103, 155, 212, 25, 65, 48, 1, 36, 82, 7, 100, 245, 133, 172, 188, 2, 93,
                128, 161, 167,
            ],
            [
                23, 27, 232, 245, 230, 59, 177, 16, 122, 171, 201, 72, 110, 139, 236, 161, 26, 133,
                16, 81, 13, 174, 83, 2, 174, 48, 133, 96, 203, 200, 40, 28, 10, 125, 58, 39, 118,
                171, 14, 66, 145, 227, 85, 158, 179, 200, 24, 238, 247, 14, 135, 228, 158, 32, 227,
                99, 148, 2, 102, 58, 35, 160, 219, 79,
            ],
            [
                27, 56, 217, 136, 79, 183, 142, 192, 199, 64, 53, 106, 19, 227, 234, 217, 97, 32,
                139, 181, 232, 114, 115, 38, 26, 148, 179, 194, 73, 234, 247, 133,
            ],
            [
                30, 51, 1, 19, 25, 27, 32, 6, 2, 13, 22, 28, 44, 18, 15, 37, 23, 10, 52, 36, 46, 9,
                11, 21, 16, 8, 50, 24, 48, 12, 4, 7, 49, 33, 42, 5, 29, 45, 38, 26, 40, 14, 47, 20,
                17, 31, 35, 43, 39, 3, 41, 34,
            ],
        );
        println!("{}", result)
    }
}
