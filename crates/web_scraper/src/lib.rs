mod exam_page;

pub const JEE_MAIN_URL: &'static str = "https://questions.examside.com/past-years/jee/jee-main";
pub const NEET_URL: &'static str = "https://questions.examside.com/past-years/medical/neet";
pub const JEE_ADVANCED_URL: &'static str = "https://questions.examside.com/past-years/jee/jee-advanced";

pub async fn run() -> Result<(), error::Error> {
    let exam_data = exam_page::extract(JEE_ADVANCED_URL).await?;
    log::info!("{:#?}", exam_data);
    Ok(())
}
