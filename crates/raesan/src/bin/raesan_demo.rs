#[tokio::main]
async fn main() {
    env_logger::Builder::new()
        .filter_module("raesan_demo", log::LevelFilter::Debug)
        .filter_module("raesan", log::LevelFilter::Debug)
        .filter_level(log::LevelFilter::Off)
        .init();

    let app_env = match std::env::var("PUBLIC_APP_ENV") {
        Ok(out) => out,
        Err(_) => {
            log::warn!("Unable to get the app environment, assuming development environment");
            String::from("development")
        }
    };
    let app = raesan::App::new("./test.db", raesan::Environment::from(app_env.as_str()))
        .await
        .unwrap();
    log::info!("{:#?}", app.get_exam_list().await.unwrap());
    log::info!("{:#?}", app.get_subject_list().await.unwrap());
    log::info!("{:#?}", app.get_chapter_list().await.unwrap());
}
