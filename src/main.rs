use chrono::Utc;
use dotenv::dotenv;
use envy;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;

#[derive(Deserialize, Debug)]
struct Config {
    database_url: String,
}

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let config: Config = envy::from_env().expect("Failed to load config from environment");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    // Create the "days" table with an auto-generated 'id' field and a 'date' field
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS days (
            id SERIAL PRIMARY KEY,
            date TEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await?;

    let current_date = Utc::now().format("%Y-%m-%d").to_string();

    // Check if today's date already exists in the days table
    let rows: (bool,) = sqlx::query_as("SELECT EXISTS(SELECT 1 FROM days WHERE date = $1)")
        .bind(&current_date)
        .fetch_one(&pool)
        .await?;

    if !rows.0 {
        // Insert today's date into the days table
        sqlx::query("INSERT INTO days (date) VALUES ($1)")
            .bind(&current_date)
            .execute(&pool)
            .await?;
    }

    // Retrieve all dates from the days table and print them
    let rows = sqlx::query_as::<_, (i32, String)>("SELECT id, date FROM days")
        .fetch_all(&pool)
        .await?;

    for row in rows {
        println!("ID: {}, Date: {}", row.0, row.1);
    }

    Ok(())
}
