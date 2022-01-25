
#[cfg(test)]
mod tests {
    use tricebot::tricebot::TriceBot;
    use tricebot::game_settings::GameSettings;

    use hyper_tls::HttpsConnector;
    use hyper::Client;
    use dotenv;

    use tokio;
    #[tokio::test]
    async fn check_auth_token() {
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

        let trice = TriceBot::new(token.clone(), api_url, extern_url);

        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        match trice.req( &client, "api/checkauthkey", token, false ).await {
            Err(_) => panic!( "Got an error back" ),
            Ok(r) => {
                assert_eq!( std::str::from_utf8(&hyper::body::to_bytes(r.into_body()).await.unwrap()).unwrap(), "valid=1" );
            },
        }
    }
}
