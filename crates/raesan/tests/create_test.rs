#[rstest::fixture]
async fn app() -> Result<raesan::App, error::Error> {
    let workspace_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or(error::Error::IOError("nope".to_string()))?
        .parent()
        .ok_or(error::Error::IOError("nope".to_string()))?;
    let db_path = workspace_root.join(format!("{}.db", raesan::constants::DB_NAME));
    raesan::App::new(
        &db_path.display().to_string(),
        raesan::Environment::from("development"),
    )
    .await
}

#[tokio::test]
#[rstest::rstest]
async fn create_test() {
    let app = app().await.unwrap();
    let selected_chapters =
        sqlx::query_as::<_, schema::Chapter>("SELECT * FROM chapter ORDER BY RANDOM() limit 5")
            .fetch_all(&app.db_pool)
            .await
            .unwrap()
            .iter()
            .map(|curr_chapter| curr_chapter.id.clone())
            .collect::<Vec<String>>();
    let test = app.create_test(50, selected_chapters).await.unwrap();
    println!("questions_per_chapters: {:#?}", test.chapter_summaries);
}
