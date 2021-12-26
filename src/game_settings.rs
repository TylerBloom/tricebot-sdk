
use serde::{Serialize,Deserialize};

#[derive(Debug,Copy,Clone,Serialize,Deserialize)]
pub struct GameSettings {
    spectators_allowed: bool,
    spectators_need_password: bool,
    spectators_can_chat: bool,
    spectators_can_see_hands: bool,
    only_registered: bool,
    player_deck_verification: bool,
}

impl GameSettings {
    pub fn new() -> Self {
        GameSettings {
            spectators_allowed: false,
            spectators_need_password: false,
            spectators_can_chat: false,
            spectators_can_see_hands: false,
            only_registered: false,
            player_deck_verification: false,
        }
    }
}
