use crate::constants;
use reqwest as req;
use tokio::sync::mpsc;

pub async fn extract(
    page_url: &str,
    log_tx: mpsc::Sender<crate::ScraperLog>,
) -> Result<serde_json::Value, error::Error> {
    let mut last_error: Option<error::Error> = None;

    for attempt in 1..=constants::MAX_FETCH_ATTEMPTS {
        match fetch_and_extract(page_url).await {
            Ok(out) => {
                return Ok(out);
            }
            Err(e) => {
                log_tx
                    .send(crate::ScraperLog::Warn(format!(
                        "Fetch attempt {}/{} failed: {}",
                        attempt,
                        constants::MAX_FETCH_ATTEMPTS,
                        e
                    )))
                    .await?;
                log::debug!("Waiting {} seconds", constants::FETCH_TIMEOUT.as_secs());
                last_error = Some(e);
                // the code below makes sures that we only sleep for the first two attempts
                if attempt < constants::MAX_FETCH_ATTEMPTS {
                    tokio::time::sleep(constants::FETCH_TIMEOUT).await;
                }
            }
        }
    }
    let error = match last_error {
        Some(e) => e,
        None => error::Error::ParseError("Just failed without an error".to_string()),
    };
    Err(error)
}

async fn fetch_and_extract(page_url: &str) -> Result<serde_json::Value, error::Error> {
    let html_string = req::get(page_url).await?.text().await?;
    let html = scraper::Html::parse_document(&html_string);
    let script_selector = scraper::Selector::parse("script")?;

    // get the <script> tag with "data:" inside it, because that is the one that contains the data
    let data_script_result = html.select(&script_selector).find(|item| {
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

    // extract the JS text between "data:" and "form: null" and remove the trailing nonsense
    let js_text = data_script
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

    // convert the JS text into a boa_engine result and parse the result into JSON
    let mut context = boa_engine::Context::default();
    let res = context.eval(boa_engine::Source::from_bytes(&format!(
        "let data = {}; data",
        js_text
    )))?;
    let parsed_json: serde_json::Value = res
        .to_json(&mut context)?
        .ok_or_else(|| error::Error::BoaEngineError("Failed to get computed JSON".to_string()))?;

    // get the root data node
    let data = parsed_json
        .as_array()
        .ok_or_else(|| {
            error::Error::DeserializeError("Failed to get the root node as an array".to_string())
        })?
        .get(1)
        .ok_or_else(|| {
            error::Error::DeserializeError("Failed to get the root[1] element".to_string())
        })?
        .get("data")
        .ok_or_else(|| {
            error::Error::DeserializeError("Failed to get the root[1][data] field".to_string())
        })?;

    Ok(data.clone())
}
