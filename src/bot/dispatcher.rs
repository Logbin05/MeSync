use super::commands::{self, Command};
use crate::bot::callbacks;
use crate::bot::callbacks::tasks::types::TaskStorage;
use teloxide::{dispatching::Dispatcher, prelude::*, types::Update};

async fn handler(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Start => commands::start::start(bot, msg).await?,
    }
    Ok(())
}

async fn callback_handler(bot: Bot, q: CallbackQuery, storage: TaskStorage) -> ResponseResult<()> {
    callbacks::handle(bot, q, storage).await
}

pub async fn run(bot: Bot, storage: TaskStorage) {
    Dispatcher::builder(
        bot,
        dptree::entry()
            .branch(
                Update::filter_message()
                    .filter_command::<crate::bot::commands::Command>()
                    .endpoint(crate::bot::dispatcher::handler)
            )
            .branch(
                Update::filter_callback_query()
                    .endpoint(crate::bot::dispatcher::callback_handler)
            )
    )
    .dependencies(dptree::deps![storage])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}