use crate::{
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
