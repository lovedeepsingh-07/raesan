pub mod content_cleaner;
pub mod from_json;
pub mod pages;

pub use from_json::chapter::from_json as chapter_from_json;
pub use from_json::exam::from_json as exam_from_json;
pub use from_json::question::{
    QuestionResult, from_json as question_from_json, get_answer as question_get_answer,
};
pub use from_json::question_option::from_json as question_option_from_json;
pub use from_json::subject::from_json as subject_from_json;

use tokio::sync::mpsc;

#[derive(Debug)]
pub struct ExamSide {}

impl ExamSide {
    pub const FETCH_PATHS: [&str; 3] = [
        "past-years/jee/jee-main",
        "past-years/medical/neet",
        "past-years/jee/jee-advanced",
    ];
}

// how this scraper is implemented is:
// - first of all we go through all the "FETCH_PATHS"
// - every fetch is an exam page that contains data about it's subjects and chapters, we fetch the
// exam page, and extract data from it using "pages::exam_page::extract", that function populates
// the database with Exam, Subjects, and Chapters using the data from that exam page, and it
// returns the exam that it fetched
// - then we fetch all the subjects from the database that belong to that exam
// - we go through each of those subjects and get their "record" from the database, it's like a key that represents
// that subject on that website, so basically "Physics" would have a record of "physics", it can be
// anything from a simple slug to a UUID, this record was inserted into the database along with the
// subject
// - then we fetch all the subjects from the database that belong to that subject
// - then we go through each of those chapters and get their "record" from the database
// - after that, create a "chapter_fetch_url" that points to the chapter page on that website,
// because for this website all the questions are on the chapter page
// - once we have the chapter page url, we fetch the page and extract the data using the
// "pages::chapter_page::extract" function, that function populates the database with all the
// Questions from that exam
impl crate::Scraper for ExamSide {
    async fn scrape(
        base_url: &str,
        db_pool: &sqlx::Pool<sqlx::Sqlite>,
        log_tx: mpsc::Sender<crate::ScraperLog>,
    ) -> Result<(), error::Error> {
        for exam_path in Self::FETCH_PATHS {
            let exam_page_metadata =
                pages::metadata::extract(&format!("{}/{}", base_url, exam_path), log_tx.clone())
                    .await?;
            let curr_exam =
                pages::exam_page::extract(db_pool, log_tx.clone(), &exam_page_metadata).await?;

            let subjects: Vec<schema::Subject> =
                sqlx::query_as::<_, schema::Subject>("SELECT * FROM subject WHERE exam_id = $1")
                    .bind(&curr_exam.id)
                    .fetch_all(db_pool)
                    .await?;

            for curr_subject in subjects {
                let subject_record: schema::SourceRecord =
                    sqlx::query_as::<_, schema::SourceRecord>(
                        "SELECT * FROM source_record WHERE entity_id = $1",
                    )
                    .bind(&curr_subject.id)
                    .fetch_one(db_pool)
                    .await?;

                let chapters: Vec<schema::Chapter> = sqlx::query_as::<_, schema::Chapter>(
                    "SELECT * FROM chapter WHERE subject_id = $1",
                )
                .bind(&curr_subject.id)
                .fetch_all(db_pool)
                .await?;

                for curr_chapter in chapters {
                    let chapter_record: schema::SourceRecord =
                        sqlx::query_as::<_, schema::SourceRecord>(
                            "SELECT * FROM source_record WHERE entity_id = $1",
                        )
                        .bind(&curr_chapter.id)
                        .fetch_one(db_pool)
                        .await?;

                    log_tx
                        .send(crate::ScraperLog::Info(format!(
                            "Fetching page data for (exam/subject/chapter): {}/{}/{}",
                            curr_exam.title, curr_subject.title, curr_chapter.title
                        )))
                        .await?;

                    let chapter_fetch_url = format!(
                        "{}/{}/{}/{}",
                        base_url, exam_path, subject_record.source_key, chapter_record.source_key
                    );
                    let chapter_page_metadata =
                        pages::metadata::extract(&chapter_fetch_url, log_tx.clone()).await?;

                    pages::chapter_page::extract(
                        db_pool,
                        log_tx.clone(),
                        &chapter_page_metadata,
                        &curr_chapter.id,
                    )
                    .await?;
                }
            }
        }

        Ok(())
    }
}
