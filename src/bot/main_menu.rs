use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, MaybeInaccessibleMessage};

pub fn main_menu_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback("⚙️ Настройки", "settings"),
            InlineKeyboardButton::callback("🎛️ Панель управления", "control_panel"),
        ],
        vec![
            InlineKeyboardButton::url(
                "🔰 Поддержка",
                "https://t.me/logbin05?direct".parse().unwrap(),
            ),
            InlineKeyboardButton::callback("📒 Политика кодифицир", "policy"),
        ],
        vec![
            InlineKeyboardButton::callback("💰 Подписка", "subscription"),
            InlineKeyboardButton::callback("🛠️ Упр.подпиской", "control_subscription"),
        ],
        vec![InlineKeyboardButton::url(
            "✏️ Обратная связь",
            "https://t.me/logbin05".parse().unwrap(),
        )],
        vec![InlineKeyboardButton::url(
            "📒 Канал разработчика",
            "https://t.me/logbin05".parse().unwrap(),
        )],
    ])
}

pub fn escape_markdown_v2(text: &str) -> String {
    let special_chars = r"_*[]()~`>#+-=|{}.!";
    let mut escaped = String::new();
    for c in text.chars() {
        if special_chars.contains(c) {
            escaped.push('\\');
        }
        escaped.push(c);
    }
    escaped
}

pub fn main_menu_text(msg: &MaybeInaccessibleMessage) -> String {
    let full_name = msg.chat().first_name().unwrap_or("Гость");
    let username = msg.chat().username().unwrap_or(full_name);
    format!(
        "{}",
        escape_markdown_v2(&format!(
            "👋 Привет, {}! Я MeSync — твой персональный помощник.",
            username
        ))
    )
}
