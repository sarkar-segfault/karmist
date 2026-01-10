use dialoguer::Input;
use serde::{Deserialize, Serialize};

pub type Database = Vec<Task>;

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub desc: String,
    pub status: bool,
}

pub fn create(db: &mut Database, id: Option<String>) {
    db.push(Task {
        id: match id {
            Some(s) => s,
            None => Input::<String>::new()
                .with_prompt("enter the id of the new task")
                .with_initial_text("my_amazing_task")
                .interact_text()
                .unwrap_or_else(|e| fatal!("failed to obtain id: {}", e)),
        },
        title: Input::<String>::new()
            .with_prompt("enter the title")
            .with_initial_text("my amazing task!")
            .interact_text()
            .unwrap_or_else(|e| fatal!("failed to obtain title: {}", e)),
        desc: Input::<String>::new()
            .with_prompt("enter the description")
            .allow_empty(true)
            .with_initial_text("this task is so amazing i can't express it in words")
            .interact_text()
            .unwrap_or_else(|e| fatal!("failed to obtain description: {}", e)),
        status: false,
    })
}
