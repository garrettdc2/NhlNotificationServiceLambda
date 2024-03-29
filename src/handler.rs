use std::env;
use std::collections::HashMap;
use lambda_runtime::{Error, LambdaEvent};
use serde::{Deserialize};
use nhl_api_client::Client;

#[derive(Deserialize)]
pub struct Request {}

pub(crate) async fn handler(_event: LambdaEvent<Request>) -> Result<String, Error> {
    let team_name = env::var("TEAM_NAME").expect("Env variable TEAM_NAME not found");
    let webhook_url = env::var("WEBHOOK_URL").expect("Env variable WEBHOOK_URL not found");
    let client = Client::new(team_name.clone());
    let resp = client.get_todays_game().await;
    match resp {
        None => Ok(format!("No games today for the {}", team_name)),
        Some(r) => {
            let mut body = HashMap::new();
            body.insert("content", r);
        
            let client = reqwest::Client::new();
            let webhook_response = client.post(webhook_url)
                .header("Content-Type", "application/json")
                .json(&body)
                .send()
                .await?;
            let serialized_response = webhook_response.text().await?;
        
            Ok(serialized_response)
        }
    }
}