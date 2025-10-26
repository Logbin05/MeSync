use teloxide::prelude::*;
use teloxide::types::CallbackQuery;

pub async fn handle(bot: Bot, q: CallbackQuery) -> ResponseResult<()> {
    if let Some(msg) = q.message {
        bot.send_message(msg.chat().id, "⚙️ В данный момент, комманда в разработке...").await?;
    }
    Ok(())
}