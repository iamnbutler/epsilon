use crate::AsyncAppContext;

#[derive(sqlx::FromRow)]
pub struct Task {
    id: i32,
    title: String,
    completed: bool,
}

impl Task {
    pub async fn new(
        cx: &AsyncAppContext,
        title: String,
        day_id: Option<i32>,
    ) -> Result<Task, sqlx::Error> {
        let task = sqlx::query_as::<_, Task>(
            "INSERT INTO tasks (title, completed, day_id) VALUES ($1, $2, $3) RETURNING id, title, completed, day_id",
        )
        .bind(&title)
        .bind(false)
        .bind(day_id)
        .fetch_one(cx.db_pool())
        .await?;

        Ok(task)
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn completed(&self) -> bool {
        self.completed
    }

    pub async fn get_by_id(id: i32, cx: &AsyncAppContext) -> Result<Task, sqlx::Error> {
        let task = sqlx::query_as::<_, Task>(
            "SELECT id, title, completed, day_id FROM tasks WHERE id = $1",
        )
        .bind(id)
        .fetch_one(cx.db_pool())
        .await?;

        Ok(task)
    }

    pub async fn set_completed(&self, cx: &AsyncAppContext) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE tasks SET completed = $1 WHERE id = $2")
            .bind(true)
            .bind(self.id)
            .execute(cx.db_pool())
            .await?;

        Ok(())
    }
}

pub struct Tasks {}

impl Tasks {
    pub async fn all() -> Result<Vec<Task>, sqlx::Error> {
        let tasks = sqlx::query_as::<_, Task>("SELECT id, title, completed FROM tasks")
            .fetch_all(crate::AsyncAppContext::new().await?.db_pool())
            .await?;

        Ok(tasks)
    }
}
