
#[cfg(test)]
mod tests {
    use tricebot::tricebot::TriceBot;
    
    use hyper::Client;
    use tokio;
    use dotenv;
    
    #[tokio::test]
    async fn check_auth_token() {
        dotenv::dotenv().ok();
        for (key, val) in dotenv::vars() {
            println!( "{}: {}", key, val );
        }
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

        let client = Client::new();
        match trice.req( &client, "api/checkauthkey", token, false ).await {
            Ok(_) => println!("Got a message back"),
            Err(_) => panic!( "Got an error back" ),
        }

    }
}
