
use crate::utils::bool_to_string;

use serde::{Serialize,Deserialize};

#[derive(Debug,Clone,Serialize,Deserialize)]
#[allow(non_snake_case)]
pub struct GameSettings {
    gamename: String,
    password: String,
    playerCount: u8,
    spectatorsAllowed: bool,
    spectatorsNeedPassword: bool,
    spectatorsCanChat: bool,
    spectatorsCanSeeHands: bool,
    onlyRegistered: bool,
    playerDeckVerification: bool,
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
        let mut digest: String = format!("gamename={})\n", self.gamename);
        digest += &format!("password={}\n", self.password);
        digest += &format!("playerCount={}\n", self.playerCount);
        digest += &format!("spectatorsAllowed={}\n", bool_to_string(self.spectatorsAllowed));
        digest += &format!("spectatorsNeedPassword={}\n", bool_to_string(self.spectatorsNeedPassword));
        digest += &format!("spectatorsCanChat={}\n", bool_to_string(self.spectatorsCanChat));
        digest += &format!("spectatorsCanSeeHands={}\n", bool_to_string(self.spectatorsCanSeeHands));
        digest += &format!("onlyRegistered={}\n", bool_to_string(self.onlyRegistered));
        digest += &format!("playerDeckVerification={}", bool_to_string(self.playerDeckVerification));
        digest
    }
}

#[cfg(test)]
mod tests {
    use super::GameSettings;

    #[test]
    fn check_bool() {
        let mut settings = GameSettings::new();
        settings.spectatorsAllowed = true;
        println!("{}", settings.to_string());
        panic!();
    }
}
