use crate::utils::bool_to_string;
use crate::trice_error::TriceError;

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
    pub fn new(game_name: String, password: String) -> Result<Self, TriceError> {
        if game_name.is_empty() {
            Err(TriceError::new(String::from("Game name is empty.")))
        } else if password.is_empty() {
            Err(TriceError::new(String::from("Password is empty.")))
        } else {
            Ok(
            GameSettings {
                gamename: game_name,
                password,
                playerCount: 2,
                spectatorsAllowed: false,
                spectatorsNeedPassword: false,
                spectatorsCanChat: false,
                spectatorsCanSeeHands: false,
                 onlyRegistered: false,
                playerDeckVerification: false,
            }
            )
        }
    }
    pub fn to_string(&self) -> String {
        let mut digest: String = format!(
            "gamename={}\n",
            self.gamename.replace("_", "").replace(" ", "")
        );
        if self.password.is_empty() {
            digest += "password=*\n";
        } else {
            digest += &format!("password={}\n", self.password);
        }
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
