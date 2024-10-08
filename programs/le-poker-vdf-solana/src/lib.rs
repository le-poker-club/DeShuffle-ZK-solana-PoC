use crate::game::Game;
use crate::zkp::verifier::verify;
use anchor_lang::prelude::*;

pub mod game;
pub mod vdf;
pub mod zkp;

declare_id!("wazzeGJsjb5kBhYDgH3BLBMNpZmiMvkCQPX8bf2GEr7");

#[program]
pub mod le_poker_vdf_solana {
    use super::*;

    pub fn verify_vdf(
        _ctx: Context<Initialize>,
        n: u64,
        time: u64,
        x: String,
        y: [u8; 32],
        pi: u64,
        challenge: String,
    ) -> Result<()> {
        let result = vdf::wes18::verify_wes_18(n, time, x, y, pi, challenge);
        require!(result, ErrorCode::VDFVerificationFailed);
        Ok(())
    }

    pub fn game_start(
        ctx: Context<NewGame>,
        game_id: String,
        n: u64,
        time: u64,
        x: String,
        player_list: Vec<String>,
        create_timestamp: u64,
    ) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.game_start(game_id, n, time, x, player_list, create_timestamp)
    }

    pub fn game_end(ctx: Context<EndGame>, y: [u8; 32], pi: u64, challenge: String) -> Result<()> {
        let game = &mut ctx.accounts.game;

        // check game state
        require!(
            game.state == game::GameState::Ongoing,
            ErrorCode::GameNotStarted
        );

        // check vdf proof
        let result = vdf::wes18::verify_wes_18(game.n, game.time, game.x.clone(), y, pi, challenge);
        require!(result, ErrorCode::VDFVerificationFailed);

        game.game_end(pi)
    }

    pub fn game_cards_verify(
        ctx: Context<EndGame>,
        y_hash: [u8; 32],
        proof_a: [u8; 64],
        proof_b: [u8; 128],
        proof_c: [u8; 64],
        cards: [u8; 52],
    ) -> Result<()> {
        let game = &mut ctx.accounts.game;

        // check game state
        require!(
            game.state == game::GameState::Ended,
            ErrorCode::GameNotEnded
        );

        // check y zkp
        let zk_result = verify(proof_a, proof_b, proof_c, y_hash, cards);
        require!(zk_result, ErrorCode::ZKPVerificationFailed);
        game.game_cards(cards)
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct NewGame<'info> {
    #[account(init, payer = owner, space = Game::MAXIMUM_SIZE)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EndGame<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("VDF Verification Failed")]
    VDFVerificationFailed,
    #[msg("ZKP Verification Failed")]
    ZKPVerificationFailed,
    #[msg("Game has not started yet")]
    GameNotStarted,
    #[msg("Game has not ended yet")]
    GameNotEnded,
    #[msg("Cards Verification Failed")]
    CardsVerificationFailed,
}
