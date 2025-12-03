use teloxide::prelude::*;
use teloxide::types::{CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup};

pub async fn handle(bot: Bot, q: CallbackQuery) -> ResponseResult<()> {
    let chat_id = q
        .message
        .as_ref()
        .map(|m| m.chat().id)
        .unwrap_or(ChatId(q.from.id.0 as i64));

    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();
    keyboard.push(vec![
        InlineKeyboardButton::callback("📒 Задачи", "tasks"),
        InlineKeyboardButton::callback("📕 Заметки", "notes"),
    ]);
    keyboard.push(vec![InlineKeyboardButton::callback("⬅️ Назад", "GoToBack")]);
    let markup = InlineKeyboardMarkup::new(keyboard);

    if let Some(msg) = q.message {
        bot.edit_message_text(chat_id, msg.id(), "🎛️ Панель управления")
            .reply_markup(markup)
            .await?;
    }
    Ok(())
}
