#[tokio::main]
async fn main() {
    env_logger::Builder::new()
        .filter_module("tester", log::LevelFilter::Debug)
        .filter_module("web_scraper", log::LevelFilter::Debug)
        .filter_level(log::LevelFilter::Off)
        .init();
    if let Err(e) = web_scraper::run().await {
        log::error!("Failed to run web scraper, {}", e.to_string());
    }
}
