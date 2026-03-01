use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;
use web_scraper::schema;

pub const JEE_MAIN_URL: &str = "https://questions.examside.com/past-years/jee/jee-main";
pub const NEET_URL: &str = "https://questions.examside.com/past-years/medical/neet";
pub const JEE_ADVANCED_URL: &str = "https://questions.examside.com/past-years/jee/jee-advanced";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::new()
        .filter_module("raesan_web_scraper", log::LevelFilter::Debug)
        .filter_module("web_scraper", log::LevelFilter::Debug)
        .filter_level(log::LevelFilter::Off)
        .init();

    let options = SqliteConnectOptions::from_str("sqlite://test.db")?.create_if_missing(true);
    let pool = SqlitePoolOptions::new().connect_with(options).await?;

    // Run migrations
    let mut conn = pool.acquire().await?;
    for migration in web_scraper::schema::get_migration_queries() {
        sqlx::query(&migration).execute(&mut *conn).await?;
    }
    drop(conn);

    let mut tx = pool.begin().await?;

    let rows: Vec<schema::Subject> = sqlx::query_as::<_, schema::Subject>(
        r#"SELECT
            subject.id,
            subject.key,
            subject.exam_id,
            exam.key as exam_key,
            subject.title
        FROM subject
        JOIN exam on subject.exam_id = exam.id"#,
    )
    .fetch_all(&mut *tx)
    .await
    .unwrap();
    for row in rows {
        log::info!("{:#?}", row);
    }

    let rows: Vec<schema::Chapter> = sqlx::query_as::<_, schema::Chapter>(
        r#"SELECT
            chapter.id,
            chapter.key,
            exam.key as exam_key,
            chapter.subject_id,
            subject.key as subject_key,
            chapter.title,
            chapter."group" FROM chapter
        JOIN subject on chapter.subject_id = subject.id
        JOIN exam on subject.exam_id = exam.id"#,
    )
    .fetch_all(&mut *tx)
    .await
    .unwrap();
    for row in rows {
        log::info!("{:#?}", row);
    }

    let rows: Vec<schema::Chapter> = sqlx::query_as::<_, schema::Chapter>(
        r#"SELECT
            chapter.id,
            chapter.key,
            exam.key as exam_key,
            chapter.subject_id,
            subject.key as subject_key,
            chapter.title,
            chapter."group" FROM chapter
        JOIN subject on chapter.subject_id = subject.id
        JOIN exam on subject.exam_id = exam.id"#,
    )
    .fetch_all(&mut *tx)
    .await
    .unwrap();
    for row in rows {
        log::info!("{:#?}", row);
    }

    tx.commit().await?;

    Ok(())
}
