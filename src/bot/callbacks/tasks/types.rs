use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use teloxide::types::ChatId;

#[derive(Clone)]
pub enum TaskStatus {
    InProgress,
    Done,
    OnHold,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TaskStatus::InProgress => "🔄 В процессе",
            TaskStatus::Done => "✅ Готово",
            TaskStatus::OnHold => "⏸️ На паузе",
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone)]
pub struct Task {
    pub name: String,
    pub status: TaskStatus,
}

impl Task {
    pub fn new(name: impl Into<String>) -> Self {
        Task {
            name: name.into(),
            status: TaskStatus::InProgress,
        }
    }
}

pub type TaskStorage = Arc<Mutex<HashMap<ChatId, Vec<Task>>>>;
pub type PendingTasks = Arc<Mutex<std::collections::HashSet<ChatId>>>;
