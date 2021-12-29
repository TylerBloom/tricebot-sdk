use crate::game_settings::GameSettings;

use hyper::{Body, Client, Response, Request};
use hyper::client::connect::HttpConnector;
use hyper_tls::HttpsConnector;

pub struct GameMade {
    success: bool,
    game_id: u64,
    replay_name: String,
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
}
/*
class TriceBot:
    # verify = false as self signed ssl certificates will cause errors here
    def req(self, urlpostfix: str, data: str, abs: bool = False) -> str:
        print(data)
        url = urlpostfix
        if not abs:
            url = f'{self.apiURL}/{url}'
        resp = requests.get(url, timeout=7.0, data=data,  verify=False).text
        if not abs:
            print(resp)
        return resp

    def reqBin(self, urlpostfix: str, data: str, abs: bool = False) -> str:
        print(data)
        url = urlpostfix
        if not abs:
            url = f'{self.apiURL}/{url}'
        resp = requests.get(url, timeout=7.0, data=data,  verify=False).content
        if not abs:
            print(resp)
        return resp

    def checkauthkey(self):
        return self.req("api/checkauthkey", self.authToken) == "1"

    def getDownloadLink(self, replayName: str) -> str:
        return f'{self.externURL}/{replayName}'

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

    def createGame(self, gamename: str, password: str, playercount: int, spectatorsallowed: bool, spectatorsneedpassword: bool, spectatorscanchat: bool, spectatorscanseehands: bool, onlyregistered: bool, playerdeckverification: bool, playernames, deckHashes):
        if len(playernames) != len(deckHashes):
            GameMade(False, -1, -1) # They must the same length dummy!

        body  = f'authtoken={self.authToken}\n'
        body += f'gamename={gamename.replace(" ", "").replace("_", "")}\n'
        body += f'password={password}\n'
        body += f'playerCount={playercount}\n'
        body += f'spectatorsAllowed={int(spectatorsallowed)}\n'
        body += f'spectatorsNeedPassword={int(spectatorsneedpassword)}\n'
        body += f'spectatorsCanChat={int(spectatorscanchat)}\n'
        body += f'spectatorsCanSeeHands={int(spectatorscanseehands)}\n'
        body += f'onlyRegistered={int(onlyregistered)}\n'
        body += f'playerDeckVerification={int(playerdeckverification)}\n'

        if playerdeckverification:
            for i in range(0, len(playernames)):
                if playernames[i] == "" or playernames[i] == None: # No name
                    body += f'playerName=*\n'
                else:
                    body += f'playerName={playernames[i]}\n'
                    if len(deckHashes[i]) == 0:
                        body += f'deckHash=*\n'
                    else:
                        for deckHash in deckHashes[i]:
                            body += f'deckHash={deckHash}\n'

        try:
            message = self.req("api/creategame", body)
            print(message)
        except OSError as exc:
            #Network issues
            print("[TRICEBOT ERROR]: Netty error")
            return GameMade(False, -1, "")

        # Check for server error
        if (message.lower() == "timeout error") or (message.lower() == "error 404") or (message.lower() == "invalid auth token"):
            #Server issues
            print("[TRICEBOT ERROR]: " + message)
            return GameMade(False, -1, "")

        # Try to parse the message
        lines = message.split("\n")
        gameID: int = -1
        replayName: str = ""

        # Parse line for line
        for line in lines:
            parts = line.split("=")

            # Check length
            if len(parts) >= 2 :
                tag = parts[0]
                value = ""
                for i in range(1, len(parts)):
                    value += parts[i]
                    if i != len(parts) - 1:
                        value += "="

                if tag == "gameid":
                    # There has to be a better way to do this
                    try:
                        gameID = int(value)
                    except:
                        # Error checked at end
                        pass
                elif tag == "replayName":
                    replayName = urllib.parse.quote(value)
                # Ignore other tags
            # Ignores lines that have no equals in them

        # Check if there was an error
        success = (gameID != -1) and (replayName != "")
        print(success)
        return GameMade(success, gameID, replayName)
    */
