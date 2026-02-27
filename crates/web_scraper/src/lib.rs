pub mod chapter_page;
pub mod error;
pub mod exam_page;
pub mod page_metadata;
pub mod schema;
pub mod tree_schema;

#[derive(Debug, Default)]
pub struct WebScraper {
    exam_store: Vec<schema::Exam>,
    subject_store: Vec<schema::Subject>,
    chapter_store: Vec<schema::Chapter>,
    question_store: Vec<schema::Question>,
}

impl WebScraper {
    pub async fn extract_exam(&mut self, exam_url: &str) -> Result<(), error::Error> {
        log::info!("Deserializing exam: {:#?}", exam_url);

        let exam_page_metadata = page_metadata::extract(exam_url).await?;
        let (exam, subject_store, chapter_store) = exam_page::extract(&exam_page_metadata)?;

        self.exam_store.push(exam);
        self.subject_store.extend(subject_store);

        for curr_chapter in chapter_store.iter() {
            let chapter_page_url = format!(
                "{}/{}/{}",
                exam_url, curr_chapter.subject_key, curr_chapter.key
            );

            let mut attempts: u8 = 0;
            while attempts < 3 {
                if attempts == 0 {
                    log::info!(
                        "Deserializing chapter page with url: {:#?}",
                        chapter_page_url.as_str()
                    );
                } else {
                    log::info!("Sleeping for 5 seconds...");
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    log::info!(
                        "Retrying deserialization of chapter page with url: {:#?}",
                        chapter_page_url.as_str()
                    );
                    break;
                }

                let chapter_page_metadata =
                    match page_metadata::extract(chapter_page_url.as_str()).await {
                        Ok(out) => out,
                        Err(e) => {
                            log::warn!(
                                "Failed to extract page metadata from url: {:#?}, {}",
                                chapter_page_url.as_str(),
                                e
                            );
                            attempts += 1;
                            continue;
                        }
                    };

                let question_store = match chapter_page::extract(&chapter_page_metadata) {
                    Ok(out) => out,
                    Err(e) => {
                        log::warn!(
                            "Failed to deserialize page metadata from url: {:#?}, {}",
                            chapter_page_url.as_str(),
                            e
                        );
                        attempts += 1;
                        continue;
                    }
                };

                self.question_store.extend(question_store);
                break;
            }

            if attempts >= 3 {
                log::error!(
                    "(Moving on...) Failed to extract page metadata from url: {:#?}",
                    chapter_page_url.as_str()
                );
                continue;
            }
        }

        self.chapter_store.extend(chapter_store);
        Ok(())
    }
}
