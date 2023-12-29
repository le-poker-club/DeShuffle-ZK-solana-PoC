use crate::errors::ZKError;
use crate::verifying_key::VERIFYINGKEY;
use groth16_solana::groth16::{Groth16Verifier};
use ark_ff::bytes::{FromBytes, ToBytes};
use std::ops::Neg;
use anchor_lang::prelude::*;

pub fn verify(
    _ctx: Context<Verify>,  
    host_data: VerifyData,
) -> Result<()> {
    require!(
        verifier(
            host_data.proof_a,
            host_data.proof_b,
            host_data.proof_c,
            host_data.public_signals
        ), 
        ZKError::BoardZKVerificationFailed
    );

    Ok(())
}

fn verifier(
    mut proof_a: [u8; 64],
    proof_b: [u8; 128],
    proof_c: [u8; 64],
    public_signals: Vec<[u8; 32]>,
) -> bool {
    type G1 = ark_ec::short_weierstrass_jacobian::GroupAffine<ark_bn254::g1::Parameters>;
    fn change_endianness(bytes: &[u8]) -> Vec<u8> {
        let mut vec = Vec::new();
        for b in bytes.chunks(32) {
            for byte in b.iter().rev() {
                vec.push(*byte);
            }
        }
        vec
    }

    let proof_a_neg_g1: G1 = <G1 as FromBytes>::read(
        &*[&change_endianness(proof_a.as_slice())[..], &[0u8][..]].concat(),
    )
    .unwrap();
    let mut proof_a_neg = [0u8; 65];
    <G1 as ToBytes>::write(&proof_a_neg_g1.neg(), &mut proof_a_neg[..]).unwrap();
    let proof_a_neg = change_endianness(&proof_a_neg[..64]).try_into().unwrap();
    
    proof_a = proof_a_neg;
    let public_signals_slices: Vec<&[u8]> = public_signals.iter().map(|array| &array[..]).collect();

    let mut verifier = Groth16Verifier::new(
        &proof_a,
        &proof_b,
        &proof_c,
        &public_signals_slices.as_slice(),
        &VERIFYINGKEY,
    )
    .unwrap();
    verifier.verify().unwrap()
}

#[derive(Accounts)]
pub struct Verify {}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct VerifyData {
    public_signals: Vec<[u8; 32]>,
    proof_a: [u8; 64],
    proof_b: [u8; 128],
    proof_c: [u8; 64],  
}