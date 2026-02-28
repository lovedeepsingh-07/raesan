pub const JEE_MAIN_URL: &str = "https://questions.examside.com/past-years/jee/jee-main";
pub const NEET_URL: &str = "https://questions.examside.com/past-years/medical/neet";
pub const JEE_ADVANCED_URL: &str = "https://questions.examside.com/past-years/jee/jee-advanced";

#[tokio::main]
async fn main() {
    env_logger::Builder::new()
        .filter_module("raesan_web_scraper", log::LevelFilter::Debug)
        .filter_module("web_scraper", log::LevelFilter::Debug)
        .filter_level(log::LevelFilter::Off)
        .init();
    let mut out = String::new();

    let exam_page_json_metadata = web_scraper::get_page_data(JEE_MAIN_URL).await.unwrap();
    let (exam, subjects, chapters) =
        web_scraper::decode_exam_page(&exam_page_json_metadata).unwrap();
    out.push_str(&format!("{:#?}", exam));
    out.push('\n');
    out.push_str(&format!("{:#?}", subjects));
    out.push('\n');
    out.push_str(&format!("{:#?}", chapters));
    out.push('\n');
    for chapter in chapters {
        let chapter_page_json_metadata = web_scraper::get_page_data(&format!(
            "{}/{}/{}",
            JEE_MAIN_URL, chapter.subject_key, chapter.key
        ))
        .await
        .unwrap();
        let questions = web_scraper::decode_chapter_page(&chapter_page_json_metadata).unwrap();
        out.push_str(&format!("{:#?}", questions));
        out.push('\n');
    }
    std::fs::write("shit.txt", out).unwrap();
}
