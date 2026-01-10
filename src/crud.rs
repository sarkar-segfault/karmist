use console::style;
use dialoguer::{Confirm, Input, theme::ColorfulTheme};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter, Result},
    usize,
};

pub type Database = Vec<Task>;

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub desc: String,
    pub status: bool,
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{} {}\n{} {}\n{} {}\n{} {}",
            style("id:").green(),
            self.id,
            style("title:").green(),
            self.title,
            style("description:").green(),
            self.desc,
            style("completed:").green(),
            if self.status { "yes" } else { "no" }
        )
    }
}

pub fn create(db: &mut Database, id: Option<String>) {
    let theme = &ColorfulTheme::default();

    let new_id = match id {
        Some(s) => s,
        None => Input::<String>::with_theme(theme)
            .with_prompt("enter the id of the new task")
            .with_initial_text("my_amazing_task")
            .interact_text()
            .unwrap_or_else(|e| fatal!("failed to obtain id: {}", e))
            .trim()
            .to_string(),
    };

    if db.iter().any(|t| t.id == new_id) {
        fatal!("task with this id already exists");
    }

    db.push(Task {
        id: new_id,
        title: Input::<String>::with_theme(theme)
            .with_prompt("enter the title")
            .with_initial_text("my amazing task!")
            .interact_text()
            .unwrap_or_else(|e| fatal!("failed to obtain title: {}", e))
            .trim()
            .to_string(),
        desc: Input::<String>::with_theme(theme)
            .with_prompt("enter the description")
            .allow_empty(true)
            .with_initial_text("this task is so amazing i can't express it in words")
            .interact_text()
            .unwrap_or_else(|e| fatal!("failed to obtain description: {}", e))
            .trim()
            .to_string(),
        status: false,
    })
}

pub fn read(db: &mut Database, id: Option<String>) {
    match id {
        Some(s) => {
            println!(
                "{}",
                db.iter()
                    .find(|t| t.id == s)
                    .unwrap_or_else(|| fatal!("could not find task with id: {}", s))
            );
        }
        None => {
            for (index, task) in db.iter().enumerate() {
                println!("{}{}", task, if index == db.len() - 1 { "" } else { "\n" })
            }
        }
    }
}

pub fn update(db: &mut Database, id: Option<String>) {
    let theme = &ColorfulTheme::default();

    let targ_id = match id {
        Some(s) => s,
        None => Input::<String>::with_theme(theme)
            .with_prompt("enter the target id")
            .with_initial_text("my_amazing_task")
            .interact_text()
            .unwrap_or_else(|e| fatal!("failed to obtain id: {}", e))
            .trim()
            .to_string(),
    };

    let targ = db
        .iter_mut()
        .find(|t| t.id == targ_id)
        .unwrap_or_else(|| fatal!("could not find task with id: {}", targ_id));

    targ.title = Input::<String>::with_theme(theme)
        .with_prompt("enter the new title")
        .with_initial_text(targ.title.clone())
        .interact_text()
        .unwrap_or_else(|e| fatal!("failed to obtain title: {}", e))
        .trim()
        .to_string();

    targ.desc = Input::<String>::with_theme(theme)
        .with_prompt("enter the new description")
        .with_initial_text(targ.desc.clone())
        .interact_text()
        .unwrap_or_else(|e| fatal!("failed to obtain description: {}", e))
        .trim()
        .to_owned();

    targ.status = Confirm::with_theme(theme)
        .with_prompt("is this task completed?")
        .default(targ.status)
        .interact()
        .unwrap_or_else(|e| fatal!("failed to obtain status: {}", e));
}

pub fn delete(db: &mut Database, id: Option<String>) {
    let theme = &ColorfulTheme::default();

    match id {
        Some(s) => {
            let ok = Confirm::with_theme(theme)
                .with_prompt("do you want to delete this task?")
                .interact()
                .unwrap_or_else(|e| fatal!("failed to obtain confirmation: {}", e));

            if ok {
                db.retain(|t| t.id != s);
            }
        }
        None => {
            let ok = Confirm::with_theme(theme)
                .with_prompt("do you want to delete the whole database?")
                .interact()
                .unwrap_or_else(|e| fatal!("failed to obtain confirmation: {}", e));

            if ok {
                db.clear();
            }
        }
    }
}
