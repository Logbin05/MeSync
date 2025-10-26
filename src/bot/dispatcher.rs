use super::callbacks;
use super::commands::{self, Command};
use teloxide::{dispatching::Dispatcher, prelude::*, types::Update};

async fn handler(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Start => commands::start::start(bot, msg).await?,
    }
    Ok(())
}

async fn callback_handler(bot: Bot, q: CallbackQuery) -> ResponseResult<()> {
    callbacks::handle(bot, q).await
}

pub async fn run(bot: Bot) {
    Dispatcher::builder(
        bot.clone(),
        dptree::entry()
            .branch(
                Update::filter_message()
                    .filter_command::<Command>()
                    .endpoint(handler),
            )
            .branch(Update::filter_callback_query().endpoint(callback_handler)),
    )
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}
