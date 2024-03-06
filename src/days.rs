use chrono::NaiveDate;

use crate::AsyncAppContext;

#[derive(sqlx::FromRow)]
pub struct Day {
    pub id: i32,
    pub date: String,
}

impl Day {
    pub async fn new(cx: &AsyncAppContext, date: NaiveDate) -> Result<Day, sqlx::Error> {
        let day_str = date.format("%Y-%m-%d").to_string();
        insert_day(cx, date).await?;

        let day = sqlx::query_as::<_, Day>("SELECT id, date FROM days WHERE date = $1")
            .bind(&day_str)
            .fetch_one(cx.db_pool())
            .await?;

        Ok(day)
    }
}

pub async fn insert_day(cx: &AsyncAppContext, day: chrono::NaiveDate) -> Result<(), sqlx::Error> {
    let pool = cx.db_pool();

    let day_str = day.format("%Y-%m-%d").to_string();

    // Check if the specified date already exists in the days table
    let rows: (bool,) = sqlx::query_as("SELECT EXISTS(SELECT 1 FROM days WHERE date = $1)")
        .bind(&day_str)
        .fetch_one(pool)
        .await?;

    if !rows.0 {
        // Insert the specified date into the days table
        sqlx::query("INSERT INTO days (date) VALUES ($1)")
            .bind(&day_str)
            .execute(pool)
            .await?;
    }

    Ok(())
}
