use anchor_lang::error_code;

#[error_code]
pub enum ZKError {
    
    GameAlreadyOver,
    NotPlayersTurn,
    OnlyHostOpeningShot,
    GameAlreadyStarted,
    BoardZKVerificationFailed,
    ShotZKVerificationFailed,
    GameNotJoinable,
    FirstTurnShouldBePlayed,
    GameStillOngoing,
    NotPlayer
}