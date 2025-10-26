use chrono::{DateTime, Duration, Utc};
use lazy_static::lazy_static;
use std::collections::HashMap;
use teloxide::types::ChatId;
use tokio::sync::Mutex;

#[derive(Clone, Debug)]
pub struct SubscriptionPlan {
    pub name: String,
    pub description: String,
    pub duration_days: i64,
    pub price: f32,
}

lazy_static! {
    pub static ref PLANS: Vec<SubscriptionPlan> = vec![
        SubscriptionPlan {
            name: "⚙️ Async".to_string(),
            description: "Что вам даст подписка Async:\n
            ・ Скрывать ваше имя\n
            ・ Создание до 100 задач, в день.\n
            ・ Высший приоритет, при обращении в тех.поддерджу.\n
            ・ Функционал подписки будет пополняться.

            Длительность подписки составляет 30 дней."
                .to_string(),
            duration_days: 30,
            price: 225.0,
        },
    ];
}

lazy_static! {
    pub static ref ACTIVE_SUBSCRIPTIONS: Mutex<HashMap<ChatId, DateTime<Utc>>> =
        Mutex::new(HashMap::new());
}

pub async fn is_active(chat_id: ChatId) -> bool {
    let subs = ACTIVE_SUBSCRIPTIONS.lock().await;
    if let Some(expire) = subs.get(&chat_id) {
        *expire > Utc::now()
    } else {
        false
    }
}

pub async fn activate(chat_id: ChatId, plan: &SubscriptionPlan) {
    let mut subs = ACTIVE_SUBSCRIPTIONS.lock().await;
    let expire_date = Utc::now() + Duration::days(plan.duration_days);
    subs.insert(chat_id, expire_date);
}
