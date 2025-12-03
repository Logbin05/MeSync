use teloxide::prelude::*;
use teloxide::types::{CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup};
use super::types::{TaskStorage};

pub async fn handle(bot: Bot, q: CallbackQuery, storage: TaskStorage) -> ResponseResult<()> {
    let chat_id = q
        .message
        .as_ref()
        .map(|m| m.chat().id)
        .unwrap_or(ChatId(q.from.id.0 as i64));

    let tasks = {
        let store = storage.lock().unwrap();
        store.get(&chat_id).cloned().unwrap_or_default()
    };

    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();

    if tasks.is_empty() {
        keyboard.push(vec![
            InlineKeyboardButton::callback("➕ Создать задачу", "create_task"),
        ]);
        keyboard.push(vec![
            InlineKeyboardButton::callback("⬅️ Назад", "GoToBack"),
        ]);

        let markup = InlineKeyboardMarkup::new(keyboard);

        let text = "📋 У вас пока нет задач.\n\nНажмите «Создать задачу», чтобы добавить первую.";

        if let Some(msg) = q.message {
            bot.edit_message_text(chat_id, msg.id(), text)
                .reply_markup(markup)
                .await?;
        } else {
            bot.send_message(chat_id, text)
                .reply_markup(markup)
                .await?;
        }
    } else {

        for (i, task) in tasks.iter().enumerate() {
            keyboard.push(vec![
                InlineKeyboardButton::callback(format!("{} {}", i + 1, task.name), format!("view_task_{}", i)),
                InlineKeyboardButton::callback("✏️", format!("edit_task_{}", i)),
                InlineKeyboardButton::callback("🗑️", format!("delete_task_{}", i)),
                InlineKeyboardButton::callback(task.status.to_string(), format!("status_task_{}", i)),
            ]);
        }

        keyboard.push(vec![
            InlineKeyboardButton::callback("➕ Создать", "create_task"),
        ]);
        keyboard.push(vec![
            InlineKeyboardButton::callback("⬅️ Назад", "GoToBack"),
        ]);

        let markup = InlineKeyboardMarkup::new(keyboard);

        if let Some(msg) = q.message {
            bot.edit_message_text(chat_id, msg.id(), "📒 Ваши задачи")
                .reply_markup(markup)
                .await?;
        } else {
            bot.send_message(chat_id, "📒 Ваши задачи")
                .reply_markup(markup)
                .await?;
        }
    }

    Ok(())
}
