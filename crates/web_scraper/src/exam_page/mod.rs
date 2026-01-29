pub mod schema;

use reqwest as req;

pub async fn extract(exam_page_url: &str) -> Result<schema::ExamData, error::Error> {
    let html_string = req::get(exam_page_url).await?.text().await?;
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
    let data_text = data_script
        .text()
        .next()
        .ok_or_else(|| {
            error::Error::ParseError("Failed to get text out of the <script> tag".to_string())
        })?
        .trim()
        .split_once("data:")
        .ok_or_else(|| {
            error::Error::ParseError(
                "Failed to split the <script> tag to get everything after 'data:'".to_string(),
            )
        })?
        .1
        .trim()
        .split_once("form: null")
        .ok_or_else(|| {
            error::Error::ParseError(
                "Failed to split the <script> tag to get everything before 'form: null'"
                    .to_string(),
            )
        })?
        .0
        .trim()
        .rsplit_once(",")
        .ok_or_else(|| {
            error::Error::ParseError(
                "Failed to remove the trailing comma from the extracted json".to_string(),
            )
        })?
        .0
        .trim();

    let js_code = format!("let data = {}; data", data_text);
    let mut context = boa_engine::Context::default();
    let res = context.eval(boa_engine::Source::from_bytes(&js_code)).unwrap();
    let json_data: serde_json::Value = res.to_json(&mut context).unwrap().unwrap();
    let schema::Root(_, schema::ExamWrapper { data: exam_data }) = serde_json::from_value(json_data).unwrap();

    Ok(exam_data)
}
