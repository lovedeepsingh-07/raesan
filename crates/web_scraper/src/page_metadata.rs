use crate::error;
use reqwest as req;

pub async fn extract(input_url: &str) -> Result<serde_json::Value, error::Error> {
    let html_string = req::get(input_url).await?.text().await?;
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
    let res = context.eval(boa_engine::Source::from_bytes(&js_code))?;
    let parsed_json: serde_json::Value = res
        .to_json(&mut context)?
        .ok_or_else(|| error::Error::BoaEngineError("Failed to get computed JSON".to_string()))?;

    let root_array = parsed_json.as_array().ok_or_else(|| {
        error::Error::DeserializeError("Failed to get the root node as an array".to_string())
    })?;
    let root_element = root_array.get(1).ok_or_else(|| {
        error::Error::DeserializeError("Failed to get the second root element".to_string())
    })?;
    let data = root_element.get("data").ok_or_else(|| {
        error::Error::DeserializeError("Failed to get the 'data' field".to_string())
    })?;

    Ok(data.clone())
}
