use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;

const EXAM_QUERY: &str = r#"INSERT INTO exam (id, key, title, "group") VALUES (?1, ?2, ?3, ?4)"#;
const SUBJECT_QUERY: &str =
    r#"INSERT INTO subject (id, key, exam_id, title) VALUES (?1, ?2, ?3, ?4)"#;
const CHAPTER_QUERY: &str =
    r#"INSERT INTO chapter (id, key, subject_id, title, "group") VALUES (?1, ?2, ?3, ?4, ?5)"#;
const QUESTION_QUERY: &str = r#"INSERT INTO question (id, chapter_id, question_type, content, answer) VALUES (?1, ?2, ?3, ?4, ?5)"#;
const QUESTION_OPTION_QUERY: &str =
    r#"INSERT INTO question_option (id, question_id, key, value) VALUES (?1, ?2, ?3, ?4)"#;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::new()
        .filter_module("raesan_web_scraper", log::LevelFilter::Debug)
        .filter_module("web_scraper", log::LevelFilter::Debug)
        .filter_level(log::LevelFilter::Off)
        .init();

    let options = SqliteConnectOptions::from_str("sqlite://test.db")?.create_if_missing(true);
    let pool = SqlitePoolOptions::new().connect_with(options).await?;

    // Run migrations
    let mut conn = pool.acquire().await?;
    for migration in schema::get_migration_queries() {
        sqlx::query(&migration).execute(&mut *conn).await?;
    }
    drop(conn);

    let mut tx = pool.begin().await?;
    let ron_data: Vec<tree_schema::T_Exam> = ron::from_str(&std::fs::read_to_string("test.ron")?)?;

    for curr_exam in ron_data {
        sqlx::query(EXAM_QUERY)
            .bind(curr_exam.id)
            .bind(curr_exam.key)
            .bind(curr_exam.title)
            .bind(curr_exam.group)
            .execute(&mut *tx)
            .await?;
        for curr_subject in curr_exam.subjects {
            sqlx::query(SUBJECT_QUERY)
                .bind(curr_subject.id)
                .bind(curr_subject.key)
                .bind(curr_subject.exam_id)
                .bind(curr_subject.title)
                .execute(&mut *tx)
                .await?;
            for curr_chapter in curr_subject.chapters {
                sqlx::query(CHAPTER_QUERY)
                    .bind(curr_chapter.id)
                    .bind(curr_chapter.key)
                    .bind(curr_chapter.subject_id)
                    .bind(curr_chapter.title)
                    .bind(curr_chapter.group)
                    .execute(&mut *tx)
                    .await?;
                for curr_question in curr_chapter.questions {
                    sqlx::query(QUESTION_QUERY)
                        .bind(curr_question.id)
                        .bind(curr_question.chapter_id)
                        .bind(curr_question.question_type)
                        .bind(curr_question.content)
                        .bind(curr_question.answer)
                        .execute(&mut *tx)
                        .await?;
                    for (_, curr_option) in curr_question.options {
                        sqlx::query(QUESTION_OPTION_QUERY)
                            .bind(curr_option.id)
                            .bind(curr_option.question_id)
                            .bind(curr_option.key)
                            .bind(curr_option.value)
                            .execute(&mut *tx)
                            .await?;
                    }
                }
            }
        }
    }

    tx.commit().await?;

    Ok(())
}
