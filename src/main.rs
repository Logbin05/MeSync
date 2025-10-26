mod bot;
mod config;

use log::*;
use config::Config;
use bot::dispatcher::run;
use teloxide::prelude::*;

type TeloxideResult<T> = std::result::Result<T, teloxide::RequestError>;

#[tokio::main]
async fn main() -> TeloxideResult<()> {
    pretty_env_logger::init();
    info!("🚀 MeSync Bot is starting...");

    let config = Config::from_env();
    let bot = Bot::new(config.token);

    run(bot).await;

    Ok(())
}
