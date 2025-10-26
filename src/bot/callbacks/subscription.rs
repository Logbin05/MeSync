use crate::bot::subscriptions::{self, PLANS};
use teloxide::prelude::*;
use teloxide::types::{CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup, ParseMode};

pub async fn handle(bot: Bot, q: CallbackQuery) -> ResponseResult<()> {
    let chat_id = q
        .message
        .as_ref()
        .map(|m| m.chat().id)
        .unwrap_or(ChatId(q.from.id.0 as i64));

    if let Some(msg) = &q.message {
        if let Some(data) = &q.data {
            if data.starts_with("subscribe_") {
                if let Ok(index) = data["subscribe_".len()..].parse::<usize>() {
                    if let Some(plan) = PLANS.get(index) {
                        if subscriptions::is_active(chat_id).await {
                            bot.answer_callback_query(q.id.clone())
                                .text("💡 У вас уже активна подписка!")
                                .await?;
                            return Ok(());
                        }

                        subscriptions::activate(chat_id, plan).await;

                        let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();
                        keyboard.push(vec![InlineKeyboardButton::callback("⬅️ Назад", "GoToBack")]);
                        let markup = InlineKeyboardMarkup::new(keyboard);

                        bot.edit_message_text(
                            chat_id,
                            msg.id(),
                            format!(
                                "✅ Подписка '{}' активирована!\n📅 Срок: {} дней\n💰 Цена: {}₽\n\n📝 {}",
                                plan.name, plan.duration_days, plan.price, plan.description
                            ),
                        )
                        .reply_markup(markup)
                        .await?;

                        bot.answer_callback_query(q.id.clone())
                            .text("Подписка активирована!")
                            .await?;
                    } else {
                        bot.answer_callback_query(q.id.clone())
                            .text("⚠️ Ошибка: такой подписки не существует.")
                            .await?;
                    }
                }
            } else {
                let mut keyboard: Vec<Vec<InlineKeyboardButton>> = Vec::new();

                for (index, plan) in PLANS.iter().enumerate() {
                    keyboard.push(vec![InlineKeyboardButton::callback(
                        format!("{} — {}₽", plan.name, plan.price),
                        format!("subscribe_{}", index),
                    )]);
                }

                keyboard.push(vec![InlineKeyboardButton::callback("⬅️ Назад", "GoToBack")]);

                let markup = InlineKeyboardMarkup::new(keyboard);

                bot.edit_message_text(
                    chat_id,
                    msg.id(),
                    "📦 *Доступные подписки:*\n\nВыберите нужный вариант 👇",
                )
                .parse_mode(ParseMode::MarkdownV2)
                .reply_markup(markup)
                .await?;
            }
        }
    }

    Ok(())
}
