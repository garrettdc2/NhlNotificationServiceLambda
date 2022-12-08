use lambda_runtime::{Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use crate::schedules;

#[derive(Deserialize)]
pub struct Request {
    team_name: String,
    date: Option<String>
}

#[derive(Serialize)]
pub struct Response {
    msg: String,
}

pub(crate) async fn handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let Request { team_name,  date } = &event.payload;
    let resp = schedules::get_todays_game_for_team(team_name, date).await;

    let resp = Response {
        msg: resp,
    };

    Ok(resp)
}