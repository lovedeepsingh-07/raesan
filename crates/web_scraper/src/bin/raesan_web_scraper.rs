use web_scraper::Scraper;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::new()
        .filter_module("raesan_web_scraper", log::LevelFilter::Debug)
        .filter_module("web_scraper", log::LevelFilter::Debug)
        .filter_level(log::LevelFilter::Off)
        .init();

    let output = web_scraper::examside::ExamSide::scrape().await?;
    log::info!("{:#?}", output);

    Ok(())
}
