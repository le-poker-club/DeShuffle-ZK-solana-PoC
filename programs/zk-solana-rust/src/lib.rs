use anchor_lang::prelude::*;
use instructions::*;

pub mod verifying_key;
pub mod errors;
pub mod instructions;

declare_id!("587z7FcneUpkwEJr829UDnUdfGfnwsiQSc2tcV6K9vDa");

#[program]
pub mod zk_solana_rust {
    use super::*;
    
    pub fn verify(
        ctx: Context<Verify>, 
        host_data: VerifyData,
    ) -> Result<()> {
        instructions::verify::verify(
            ctx, 
            host_data,
        )
    }
}


pub mod verify_test;