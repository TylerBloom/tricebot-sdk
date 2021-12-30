use crate::utils::bool_to_string;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct GameSettings {
    pub gamename: String,
    pub password: String,
    pub playerCount: u8,
    pub spectatorsAllowed: bool,
    pub spectatorsNeedPassword: bool,
    pub spectatorsCanChat: bool,
    pub spectatorsCanSeeHands: bool,
    pub onlyRegistered: bool,
    pub playerDeckVerification: bool,
}

impl GameSettings {
    pub fn new() -> Self {
        GameSettings {
            gamename: "Game".to_string(),
            password: "".to_string(),
            playerCount: 2,
            spectatorsAllowed: false,
            spectatorsNeedPassword: false,
            spectatorsCanChat: false,
            spectatorsCanSeeHands: false,
            onlyRegistered: false,
            playerDeckVerification: false,
        }
    }

    pub fn to_string(&self) -> String {
        let mut digest: String = format!(
            "gamename={})\n",
            self.gamename.replace("_", "").replace(" ", "")
        );
        digest += &format!("password={}\n", self.password);
        digest += &format!("playerCount={}\n", self.playerCount);
        digest += &format!(
            "spectatorsAllowed={}\n",
            bool_to_string(self.spectatorsAllowed)
        );
        digest += &format!(
            "spectatorsNeedPassword={}\n",
            bool_to_string(self.spectatorsNeedPassword)
        );
        digest += &format!(
            "spectatorsCanChat={}\n",
            bool_to_string(self.spectatorsCanChat)
        );
        digest += &format!(
            "spectatorsCanSeeHands={}\n",
            bool_to_string(self.spectatorsCanSeeHands)
        );
        digest += &format!("onlyRegistered={}\n", bool_to_string(self.onlyRegistered));
        digest += &format!(
            "playerDeckVerification={}",
            bool_to_string(self.playerDeckVerification)
        );
        digest
    }
}

#[cfg(test)]
mod tests {
    use super::GameSettings;
}
