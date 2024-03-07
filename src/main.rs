use chrono::Utc;
use dotenv::dotenv;
use envy;
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::{
    days::Day,
    tasks::{Task, Tasks},
};

mod days;
mod tasks;

#[derive(Deserialize, Debug)]
struct Config {
    database_url: String,
}

pub struct AsyncAppContext {
    pub db_pool: PgPool,
}

impl AsyncAppContext {
    pub async fn new() -> Result<AsyncAppContext, sqlx::Error> {
        dotenv().ok();
        let config: Config = envy::from_env().expect("Failed to load config from environment");
        let db_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.database_url)
            .await?;

        Ok(AsyncAppContext { db_pool })
    }

    pub fn db_pool(&self) -> &PgPool {
        &self.db_pool
    }
}

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    let cx = AsyncAppContext::new().await?;

    let today = Day::new(&cx, Utc::now().date_naive()).await?;

    Task::new(&cx, "Do something".to_string(), Some(today.id())).await?;
    Task::new(&cx, "Do something else".to_string(), None).await?;

    let task_2 = Task::get_by_id(2, &cx).await?;
    task_2.set_completed(&cx).await?;

    let tasks = Tasks::all().await?;

    println!("Today's date: {}", today.date());
    println!("Today's ID: {}", today.id());

    for task in tasks {
        println!(
            "{}: Task Title: {} ({})",
            task.id(),
            task.title(),
            task.completed()
        );
    }

    let today_tasks = today.get_tasks(&cx).await?;

    for task in today_tasks {
        println!("Today's Task: {} ({})", task.title(), task.completed());
    }

    Ok(())
}
