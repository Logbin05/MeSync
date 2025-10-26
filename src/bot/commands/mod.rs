use teloxide::utils::command::{BotCommands};

pub mod start;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "📜 Список доступных команд:"
)]
pub enum Command {
    #[command(description = "🚀 Запустить бота")]
    Start,
}
