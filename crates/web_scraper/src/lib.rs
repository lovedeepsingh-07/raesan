use reqwest as req;

const URL: &'static str = "https://questions.examside.com/past-years/jee/jee-main";

pub async fn run() -> Result<(), error::Error> {
    let html_string = req::get(URL).await?.text().await?;
    let html_doc = scraper::Html::parse_document(&html_string);
    let script_selector = scraper::Selector::parse("script")?;

    let data_script_result = html_doc.select(&script_selector).find(|item| {
        item.text()
            .next()
            .is_some_and(|script_text| script_text.contains("data:"))
    });
    let data_script = data_script_result
        .ok_or_else(|| {
            error::Error::ParseError(
                "Failed to get <script> tag with 'data:' inside it, maybe the website structure was updated"
                    .to_string(),
            )
        })?;
    let data_script_text = data_script.text().next().ok_or_else(|| {
        error::Error::ParseError("Failed to get text out of the <script> tag".to_string())
    })?.trim();

    log::debug!("{:#?}", data_script_text);
    Ok(())
}
