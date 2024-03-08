use crate::{
    completable::Completable,
    repeatable::{RepeatFrequency, Repeatable},
    AsyncAppContext,
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Ritual {
    /// The ritual's unique identifier
    id: i32,
    /// The ritual's title
    title: String,
    /// The frequency a ritual repeats
    frequency: Option<RepeatFrequency>,
    /// Whether the ritual is active
    ///
    /// Inactive rituals are not created for new days
    active: bool,
}

impl Repeatable for Ritual {
    fn frequency(&self) -> Option<&RepeatFrequency> {
        self.frequency.as_ref()
    }

    fn set_frequency(&mut self, frequency: Option<RepeatFrequency>) {
        self.frequency = frequency;
    }
}

impl Ritual {
    pub async fn new(cx: &AsyncAppContext, title: String) -> Result<Ritual, sqlx::Error> {
        let ritual = sqlx::query_as::<_, Ritual>(
            "INSERT INTO rituals (title, active) VALUES ($1, $2) RETURNING id, title, frequency, active",
        )
        .bind(&title)
        .bind(true)
        .fetch_one(cx.db_pool())
        .await?;

        Ok(ritual)
    }

    pub async fn get(cx: &AsyncAppContext, id: i32) -> Result<Ritual, sqlx::Error> {
        let ritual = sqlx::query_as::<_, Ritual>(
            "SELECT id, title, frequency, active FROM rituals WHERE id = $1",
        )
        .bind(id)
        .fetch_one(cx.db_pool())
        .await?;

        Ok(ritual)
    }

    pub async fn update(&mut self, cx: &AsyncAppContext) -> Result<Ritual, sqlx::Error> {
        sqlx::query("UPDATE rituals SET title = $1, frequency = $2, active = $3 WHERE id = $4")
            .bind(&self.title)
            .bind(&self.frequency)
            .bind(self.active)
            .bind(self.id)
            .execute(cx.db_pool())
            .await?;

        // TODO: Don't love this approach, there must be a more elegant way to do this
        let updated_ritual = Ritual::get(cx, self.id).await?;

        self.title = updated_ritual.title;
        self.frequency = updated_ritual.frequency;
        self.active = updated_ritual.active;

        Ok(self.clone())
    }

    pub async fn delete(cx: &AsyncAppContext, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM rituals WHERE id = $1")
            .bind(id)
            .execute(cx.db_pool())
            .await?;

        Ok(())
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DayRitual {
    day_id: i32,
    ritual_id: i32,
    completed: bool,
}

impl Completable for DayRitual {
    fn completed(&self) -> bool {
        self.completed
    }

    fn set_completed(&mut self, completed: bool) {
        self.completed = completed;
    }
}

impl DayRitual {
    pub async fn new(
        cx: &AsyncAppContext,
        day_id: i32,
        ritual_id: i32,
    ) -> Result<DayRitual, sqlx::Error> {
        let day_ritual = sqlx::query_as::<_, DayRitual>(
            "INSERT INTO day_rituals (day_id, ritual_id, completed) VALUES ($1, $2, $3) RETURNING day_id, ritual_id, completed",
        )
        .bind(day_id)
        .bind(ritual_id)
        .bind(false)
        .fetch_one(cx.db_pool())
        .await?;

        Ok(day_ritual)
    }

    pub async fn get(
        cx: &AsyncAppContext,
        day_id: i32,
        ritual_id: i32,
    ) -> Result<DayRitual, sqlx::Error> {
        let day_ritual = sqlx::query_as::<_, DayRitual>(
            "SELECT day_id, ritual_id, completed FROM day_rituals WHERE day_id = $1 AND ritual_id = $2",
        )
        .bind(day_id)
        .bind(ritual_id)
        .fetch_one(cx.db_pool())
        .await?;

        Ok(day_ritual)
    }

    pub async fn update(&mut self, cx: &AsyncAppContext) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE day_rituals SET completed = $1 WHERE day_id = $2 AND ritual_id = $3")
            .bind(self.completed)
            .bind(self.day_id)
            .bind(self.ritual_id)
            .execute(cx.db_pool())
            .await?;

        Ok(())
    }

    pub async fn delete(
        cx: &AsyncAppContext,
        day_id: i32,
        ritual_id: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM day_rituals WHERE day_id = $1 AND ritual_id = $2")
            .bind(day_id)
            .bind(ritual_id)
            .execute(cx.db_pool())
            .await?;

        Ok(())
    }
}
