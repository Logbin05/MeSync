mod bot;
mod config;

use std::{collections::HashMap, sync::{Arc, Mutex}};

use log::*;
use config::Config;
use bot::dispatcher::run;
use teloxide::prelude::*;

use crate::bot::callbacks::tasks::types::TaskStorage;

type TeloxideResult<T> = std::result::Result<T, teloxide::RequestError>;

#[tokio::main]
async fn main() -> TeloxideResult<()> {
    pretty_env_logger::init();
    info!("🚀 MeSync Bot is starting...");

    let config = Config::from_env();
    let bot = Bot::new(config.token);
    let storage: TaskStorage = Arc::new(Mutex::new(HashMap::new()));

    run(bot, storage).await;

    Ok(())
}
