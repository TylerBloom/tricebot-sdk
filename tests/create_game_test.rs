#[cfg(test)]
mod tests {

    use tricebot::game_settings::GameSettings;
    use tricebot::tricebot::TriceBot;

    use dotenv;
    use hyper::Client;
    use hyper_tls::HttpsConnector;
    use tokio;

    #[tokio::test]
    pub async fn create_game() {
        dotenv::dotenv().ok();
        let token = match dotenv::var("AUTH_TOKEN") {
            Ok(val) => val,
            Err(_) => panic!("Could not find an auth token in the env variables."),
        };
        let api_url = match dotenv::var("API_URL") {
            Ok(val) => val,
            Err(_) => panic!("Could not find an api url in the env variables."),
        };
        let extern_url = match dotenv::var("EXTERN_URL") {
            Ok(val) => val,
            Err(_) => panic!("Could not find an extern url in the env variables."),
        };

        let trice = TriceBot::new(token, api_url, extern_url);

        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let mut settings = GameSettings::new("Test/Game".to_string(), "password".to_string()).unwrap();
        settings.gamename = "SDK Test".to_string();
        let game_made = trice
            .create_game(&client, settings, Vec::new(), Vec::new())
            .await;
        assert!(game_made.success);
        let did_end_game = trice.end_game(&client, game_made.game_id).await.is_ok();
        assert!(did_end_game);
    }
}
