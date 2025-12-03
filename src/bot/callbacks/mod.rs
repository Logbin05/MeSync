pub mod control_panel;
pub mod control_subscription;
pub mod policy;
pub mod settings;
pub mod subscription;
pub mod tasks;

use crate::bot::main_menu::{main_menu_keyboard, main_menu_text};
use crate::bot::callbacks::tasks::types::TaskStorage;
use teloxide::prelude::*;
use teloxide::types::{CallbackQuery, ParseMode};

pub async fn handle(bot: Bot, q: CallbackQuery, storage: TaskStorage) -> ResponseResult<()> {
    if let Some(data) = &q.data {
        if data.starts_with("subscribe_") {
            return crate::bot::callbacks::subscription::handle(bot, q).await;
        }
    }

    match q.data.as_deref() {
        Some("settings") => settings::handle(bot, q).await?,
        Some("control_panel") => control_panel::handle(bot, q).await?,
        Some("policy") => policy::handle(bot, q).await?,
        Some("subscription") => subscription::handle(bot, q).await?,
        Some("control_subcription") => control_subscription::handle(bot, q).await?,
        Some("tasks") => tasks::handle(bot, q, storage).await?,
        Some("GoToBack") => {
            if let Some(msg) = &q.message {
                bot.edit_message_text(msg.chat().id, msg.id(), &main_menu_text(msg))
                    .parse_mode(ParseMode::MarkdownV2)
                    .reply_markup(main_menu_keyboard())
                    .await?;
            } else {
                bot.answer_callback_query(q.id.clone())
                    .text("⚠️ Сообщение недоступно")
                    .await?;
            }
        }

        _ => {
            bot.answer_callback_query(q.id.clone())
                .text("⚙️ Неизвестная команда")
                .await?;
        }
    }
    Ok(())
}
