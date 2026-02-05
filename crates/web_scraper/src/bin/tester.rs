pub const JEE_MAIN_URL: &'static str = "https://questions.examside.com/past-years/jee/jee-main";
pub const NEET_URL: &'static str = "https://questions.examside.com/past-years/medical/neet";

#[tokio::main]
async fn main() {
    "https://questions.examside.com/past-years/jee/jee-advanced";
    env_logger::Builder::new()
        .filter_module("tester", log::LevelFilter::Debug)
        .filter_module("web_scraper", log::LevelFilter::Debug)
        .filter_level(log::LevelFilter::Off)
        .init();
    let mut web_scraper = web_scraper::WebScraper::default();

    if let Err(e) = web_scraper.extract_exam(JEE_MAIN_URL).await {
        log::error!("Failed to run web scraper, {}", e.to_string());
    }
    std::fs::write("shit.txt", format!("{:#?}", web_scraper)).unwrap();
}
