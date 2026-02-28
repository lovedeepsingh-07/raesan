use sqlx::{sqlite, Row};

#[tokio::main]
async fn main() {
    env_logger::Builder::new()
        .filter_module("raesan_web_scraper", log::LevelFilter::Debug)
        .filter_module("web_scraper", log::LevelFilter::Debug)
        .filter_level(log::LevelFilter::Off)
        .init();

    let pool = sqlite::SqlitePool::connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::migrate!("../../migrations").run(&pool).await.unwrap();

    let mut conn = pool.acquire().await.unwrap();

    sqlx::query(&format!(
        "INSERT INTO users ( id, name ) VALUES ( '{}', '{}' )",
        "this_is_id", "this_is_name"
    ))
    .execute(&mut *conn)
    .await
    .unwrap();

    let rows = sqlx::query("SELECT * FROM users").fetch_all(&mut *conn).await.unwrap();
    for row in rows {
        let id: String = row.get("id");
        let name: String = row.get("name");
        log::info!("({}, {})", id, name);
    }
}
