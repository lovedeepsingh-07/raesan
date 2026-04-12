use sqlx::sqlite;
use std::str::FromStr;

#[derive(Debug)]
pub struct App {
    pub env: crate::Environment,
    pub db_pool: sqlite::SqlitePool,
}

impl App {
    pub async fn new(db_path: &str, env: crate::Environment) -> Result<Self, error::Error> {
        let db_options = sqlite::SqliteConnectOptions::from_str(db_path)?.create_if_missing(false);
        let db_pool = sqlite::SqlitePoolOptions::new()
            .connect_with(db_options)
            .await?;

        Ok(Self { env, db_pool })
    }

    pub async fn get_filter_metadata(&self) -> Result<Vec<schema::Exam>, error::Error> {
        let mut exams: Vec<schema::Exam> = sqlx::query_as::<_, schema::Exam>("SELECT * FROM exam")
            .fetch_all(&self.db_pool)
            .await?;

        for curr_exam in exams.iter_mut() {
            let mut subjects: Vec<schema::Subject> =
                sqlx::query_as::<_, schema::Subject>("SELECT * FROM subject WHERE exam_id = $1")
                    .bind(&curr_exam.id)
                    .fetch_all(&self.db_pool)
                    .await?;

            for curr_subject in subjects.iter_mut() {
                let chapters: Vec<schema::Chapter> = sqlx::query_as::<_, schema::Chapter>(
                    "SELECT * FROM chapter WHERE subject_id = $1",
                )
                .bind(&curr_subject.id)
                .fetch_all(&self.db_pool)
                .await?;
                curr_subject.chapters = chapters;
            }

            curr_exam.subjects = subjects;
        }
        Ok(exams)
    }

    pub async fn get_chapter_data(
        &self,
        chapter_id: &str,
    ) -> Result<schema::Chapter, error::Error> {
        let mut curr_chapter: schema::Chapter =
            sqlx::query_as::<_, schema::Chapter>("SELECT * FROM chapter WHERE id = $1")
                .bind(chapter_id)
                .fetch_one(&self.db_pool)
                .await?;

        curr_chapter.questions =
            sqlx::query_as::<_, schema::Question>("SELECT * FROM question WHERE chapter_id = $1")
                .bind(chapter_id)
                .fetch_all(&self.db_pool)
                .await?;

        for curr_question in curr_chapter.questions.iter_mut() {
            curr_question.options = sqlx::query_as::<_, schema::QuestionOption>(
                "SELECT * FROM question_option WHERE question_id = $1",
            )
            .bind(&curr_question.id)
            .fetch_all(&self.db_pool)
            .await?;
        }

        Ok(curr_chapter)
    }
}
