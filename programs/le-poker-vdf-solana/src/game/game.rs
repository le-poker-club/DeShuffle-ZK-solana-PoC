use anchor_lang::prelude::*;

#[account]
pub struct Game {
    pub game_id: String,
    pub player_list: Vec<String>,
    pub n: u64,
    pub time: u64,
    pub x: String,
    pub pi: u64,
    pub create_timestamp: u64,
    pub cards: [u8; 52],
    pub state: GameState,
}

impl Game {
    pub const MAXIMUM_SIZE: usize = 500;

    pub fn game_start(
        &mut self,
        game_id: String,
        n: u64,
        time: u64,
        x: String,
        player_list: Vec<String>,
        create_timestamp: u64,
    ) -> Result<()> {
        self.game_id = game_id;
        self.n = n;
        self.time = time;
        self.x = x;
        self.player_list = player_list;
        self.create_timestamp = create_timestamp;
        self.state = GameState::Ongoing;

        Ok(())
    }

    pub fn game_end(
        &mut self, 
        pi: u64,
    ) -> Result<()> {
        self.pi = pi;
        self.state = GameState::Ended;
        Ok(())
    }

    pub fn game_cards(
        &mut self, 
        cards: [u8; 52],
    ) -> Result<()> {
        self.cards = cards;
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum GameState {
    Ongoing,
    Ended,
}