use lambda_runtime::{service_fn, Error};
use log::LevelFilter;

use simple_logger::SimpleLogger;

mod schedules;
mod dates;
mod games;
mod handler;
mod league_records;
mod venues;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();

    let func = service_fn(handler::handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

