use anchor_lang::solana_program::hash::hash;
use num_bigint::BigUint;

pub fn verify_wes_18(
    n: u64,
    time: u64,
    x: String,
    y: [u8; 32],
    pi: u64,
    challenge: String,
) -> bool {
    let n_int = BigUint::from(n);
    let time_int = BigUint::from(time);
    let pi_int = BigUint::from(pi);
    let x_int = BigUint::parse_bytes(x.as_bytes(), 10).expect("Failed to parse BigUint");
    let challenge_int =
        BigUint::parse_bytes(challenge.as_bytes(), 10).expect("Failed to parse BigUint");

    let residue = BigUint::from(2_u32).modpow(&time_int, &challenge_int);
    let left = pi_int.modpow(&challenge_int, &n_int);
    let right = x_int.modpow(&residue, &n_int);

    let result = (left * right) % &n_int;

    let hash_result = hash(&result.to_bytes_be());

    if hash_result.to_bytes() != y {
        return false;
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vdf_verify_success() {
        let result: bool = verify_wes_18(
            92470579,
            417337200,
            "2065743662758022685238145802486066419409038405748".to_string(),
            [
                2, 170, 46, 188, 109, 112, 192, 221, 160, 72, 188, 114, 73, 79, 88, 253, 158, 218,
                34, 26, 104, 39, 159, 110, 150, 84, 14, 147, 199, 212, 48, 100,
            ],
            88035170,
            "7230909640199066306989126415855635493"
                .to_string(),
        );
        assert_eq!(result, true);
    }
}
