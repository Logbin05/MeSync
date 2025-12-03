use reqwest::Url;
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub async fn start(bot: Bot, msg: Message) -> ResponseResult<()> {
    let full_name = msg.chat.first_name().unwrap_or("Гость");
    let username = msg.chat.username().unwrap_or(full_name);
    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("⚙️ Настройки", "settings"),
            InlineKeyboardButton::callback("🎛️ Панель управления", "control_panel"),
        ],
        vec![
            InlineKeyboardButton::url(
                "🔰 Поддержка",
                "https://t.me/logbin_ov?direct".parse().unwrap(),
            ),
            InlineKeyboardButton::callback("📒 Политика кодифицир", "policy"),
        ],
        vec![
            InlineKeyboardButton::callback("💰 Подписка", "subscription"),
            InlineKeyboardButton::callback("🛠️ Управление подпиской", "control_subscription"),
        ],
        vec![InlineKeyboardButton::url(
            "✏️ Обратная связь",
            "https://t.me/logbin_ov".parse().unwrap(),
        )],
        vec![InlineKeyboardButton::url(
            "📒 Канал разработчика",
            Url::parse("https://t.me/logbin_ov").unwrap(),
        )],
    ]);

    bot.send_message(
        msg.chat.id,
        format!(
            "👋 Привет, {}! Я MeSync — твой персональный помощник.",
            username
        ),
    )
    .reply_markup(keyboard)
    .await?;
    Ok(())
}
