use crate::game_settings::GameSettings;

use urlparse;
use hyper::{Body, Client, Response, Request};
use hyper::client::connect::HttpConnector;
use hyper_tls::HttpsConnector;

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

    pub async fn req(&self, client: &Client<HttpsConnector<HttpConnector>, Body>, url_postfix: &str, body: String, abs: bool) -> Result<Response<Body>, hyper::Error> {
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

        if let Ok(response) = self.req(client, "api/creategame", body, false).await {
            let mut game_id: u64 = u64::MAX;
            let mut replay_name: String = String::new();
            let body_bytes = hyper::body::to_bytes(response.into_body()).await;
            if let Ok(lines) = std::str::from_utf8(&body_bytes.unwrap()) {
                println!( "{}", lines );
                for line in lines.split("\n") {
                    let mut parts = line.splitn(1, "=");
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
}
/*
class TriceBot:

    # Returns the zip file which contains all of the downloaded files
    # Returns none if the zip file would be empty or if there was an IOError
    def downloadReplays(self, replayURLs, replaysNotFound = []):
        # Download all the replays
        replayStrs = []
        replayNames = []

        # Iterate over each replay url
        for replayURL in replayURLs:
            try:
                res = self.reqBin(replayURL.replace(self.externURL, self.apiURL), "", abs=True)
                split = replayURL.split("/")
                name = urllib.parse.unquote(split[len(split) - 1])
                try:
                    if res.decode() == "error 404" or re.match("Not found \[.*\]", res.decode()) or re.match("<!DOCTYPE html>.*", res.decode()) or re.match("<html>.*", res.decode()):
                        # Error file not found
                        replaysNotFound.append(name)
                        #print(res == "error 404")
                        #print(re.match("Not found \[.*\]", res))
                        #print(re.match("<!DOCTYPE html>.*", res))
                    else:
                        # Create a temp file and write the data
                        replayStrs.append(res)
                        replayNames.append(name)
                except UnicodeDecodeError as e:
                    print(e) # This means we got binary :)
                    # Create a temp file and write the data
                    replayStrs.append(res)
                    replayNames.append(name)
            except OSError as exc:
                # Network issues
                print("[TRICEBOT ERROR]: Netty error")
                replaysNotFound.append(replayURL)

        # Create zip file then close the temp files
        try:
            if (len(replayStrs) == 0):
                return None
            tmpFile = tempfile.TemporaryFile(mode="wb+", suffix="tricebot.py", prefix="replaydownloads.zip")
            #tmpFile = open("I hate python.zip", "wb+")
            zipf = zipfile.ZipFile(tmpFile, "w", zipfile.ZIP_DEFLATED)
            for i in range(0, len(replayStrs)):
                replayStr = replayStrs[i]
                name = replayNames[i]
                zipf.writestr(name, replayStr, compress_type=zipfile.ZIP_DEFLATED, compresslevel=9)
            zipf.close()
            tmpFile.seek(0)
            return tmpFile
        except IOError as exc:
            print(exc)
            return None

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
