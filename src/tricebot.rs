use crate::game_settings::GameSettings;
use crate::utils::response_into_string;

use urlparse;
use tempfile::tempfile;
use hyper::{Body, Client, Response, Request};
use hyper::client::connect::HttpConnector;
use hyper_tls::HttpsConnector;

use std::collections::HashMap;

#[derive(Debug,Clone)]
pub struct GameMade {
    pub success: bool,
    pub game_id: u64,
    pub replay_name: String,
}

impl GameMade {
    pub fn new(success: bool, game_id: u64, replay_name: String) -> Self {
        GameMade {
            success,
            game_id,
            replay_name,
        }
    }
}

pub struct TriceBot {
    auth_token: String,
    api_url: String,
    extern_url: String,
}

impl TriceBot {
    pub fn new(auth_token: String, api_url: String, mut extern_url: String) -> TriceBot {
        if extern_url.is_empty() {
            extern_url = api_url.clone();
        }

        TriceBot {
            auth_token,
            api_url,
            extern_url,
        }
    }

    pub async fn req(&self, client: &Client<HttpsConnector<HttpConnector>, Body>, url_postfix: &str, body: &str, abs: bool) -> Result<Response<Body>, hyper::Error> {
        let url: String = if abs {
            url_postfix.to_string()
        } else {
            format!("https://{}/{}", self.api_url, url_postfix)
        };
        client.request(
            Request::builder()
                .method("GET")
                .uri(url)
                .body(Body::from(body))
                .unwrap(),
        ).await
    }
    
    fn download_link(&self, replay_name: &str) -> String {
        format!("{}/{}", self.extern_url, replay_name)
    }
    
    pub async fn create_game(&self, client: &Client<HttpsConnector<HttpConnector>, Body>, settings: GameSettings, player_names: Vec<String>, deck_hashes: Vec<Vec<String>>) -> GameMade {
        let mut digest = GameMade::new( false, u64::MAX, String::new() );
        if player_names.len() != deck_hashes.len() {
            return digest
        }

        let mut body  = format!("authtoken={}\n", self.auth_token);
        body += &settings.to_string();

        if settings.playerDeckVerification {
            for (i,name) in player_names.iter().enumerate() {
                if name.is_empty() {
                    body += "playerName=*\n";
                } else {
                    body += &format!("playerName={}\n", name);
                    if deck_hashes[i].is_empty() {
                        body += "deckHash=*\n";
                    } else {
                        for hash in deck_hashes[i].iter() {
                            body += &format!("deckHash={}\n", hash);
                        }
                    }
                }
            }
        }
        
        println!( "{}\n", body );

        if let Ok(response) = self.req(client, "api/creategame", body, false).await {
            let mut game_id: u64 = u64::MAX;
            let mut replay_name: String = String::new();
            if let Ok(lines) = response_into_string(response.into_body()).await {
                println!( "{}", lines );
                for line in lines.split("\n") {
                    let mut parts = line.splitn(2, "=");
                    let tag = parts.next().unwrap(); // This will always exist
                    if let Some(value) = parts.next() {
                        if tag == String::from("gameid") {
                            match value.parse::<u64>() {
                                Ok(v) => { game_id = v; },
                                Err(_) => { continue; },
                            }
                        } else if tag == String::from("replayName") {
                            match urlparse::quote(value, b"") {
                                Ok(v) => { replay_name = v; },
                                Err(_) => { continue; }
                            }
                        }
                    }
                }
            }
            if game_id != u64::MAX && !replay_name.is_empty() {
                digest = GameMade::new(true, game_id, replay_name);
            }
        }
        digest
    }
    
    pub async fn end_game(&self, client: &Client<HttpsConnector<HttpConnector>, Body>, game_id: u64) -> Result<()> {
        let body = format!( "authtoken={}\ngameid={}", self.auth_token, game_id );
        let response = self.req(client, "api/endgame", body, false).await?;
        match response_into_string(response.into_body()).await {
            Ok(s) => if s == "success" { Ok(()) } else { Err(()) },
            Err(e) => Err(e)
        }
    }
    
    pub async fn download_replays(&self, client: &Client<HttpsConnector<HttpConnector>, Body>, urls: &Vec<String>) -> HashMap<String, tempfile> {
        let mut digest = HashMap::with_capacity(urls.len());
        for url in urls {
            let mut replay_name: String;
            match urlparse::unquote(url.split("/").next(), b"") {
                Err(_) => { continue; },
                Ok(v) => { replay_name = v; }
            }
            if let Ok(response) = self.req(client, &url.replace(self.extern_url.clone(), &self.api_url), "", true).await {
                if let Ok(content) = response_into_string(response.into_body()).await {
                    if !was_bad_request(content) && let Ok(f) = tempfile() {
                        f.write(content);
                        digest.append(replay_name, f);
                    }
                }
            }
        }
        digest
    }
}
/*
class TriceBot:

    # Returns the zip file which contains all of the downloaded files
    # Returns none if the zip file would be empty or if there was an IOError

    # Returns:
    # 1 if the operation was a success
    # 2 if the slot was occupied (warns the admin that a player may need to be kicked)

    # 0 if a network error occurred
    # -1 if the game was not found
    # -2 if the player slot was not found
    def changePlayerInfo(self, gameID: int, oldPlayerName: str, newPlayerName: str):
        body  = f'authtoken={self.authToken}\n'
        body += f'oldplayername={oldPlayerName}\n'
        body += f'newplayername={newPlayerName}\n'
        body += f'gameid={gameID}'

        res = ""
        try:
            res = self.req("api/updateplayerinfo", body)
        except OSError as exc:
            #Network issues
            print("[TRICEBOT ERROR]: Netty error")
            res = "network error"

        if res == "success":
            return 1
        elif res == "success but occupied":
            return 2
        elif res == "error game not found":
            return -1
        elif res == "error player not found":
            return -2
        else:
            return 0

    # 1 if success
    # 0 auth token is bad, error 404 or network issue
    # -1 game not found
    def disablePlayerDeckVerificatoin(self, gameID: str) -> int:
        body  = f'authtoken={self.authToken}\n'
        body += f'gameid={gameID}'

        res = ""
        try:
            res = self.req("api/disableplayerdeckverification", body)
        except OSError as exc:
            #Network issues
            print("[TRICEBOT ERROR]: Netty error")
            res = "network error"
            return 0

        if res == "success":
            return 1
        elif res == "error 404" or "invalid auth token":
            return 0
        elif res == "game not found":
            return -1
        return 0

    #  1 if success
    #  0 auth token is bad or error404 or network issue
    # -1 if player not found
    # -2 if an unknown error occurred
    def kickPlayer(self, gameID: int, name: str) -> int:
        body  = f'authtoken={self.authToken}\n'
        body += f'gameid={gameID}\n'
        body += f'target={name}'

        try:
            message = self.req("api/kickplayer", body)
        except OSError as exc:
            # Network issues
            print("[TRICEBOT ERROR]: Netty error")
            return 0

        # Check for server error
        if (message == "timeout error" or message == "error 404" or message == "invalid auth token"):
            return 0
        if (message == "success"):
            return 1
        elif (message == "error not found"):
            return -1

        return -2

    */
