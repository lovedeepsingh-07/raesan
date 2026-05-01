use sqlx::sqlite;
use std::str::FromStr;
// use rand::{self, seq::SliceRandom, Rng};
use rand::RngExt;

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
    pub async fn create_test(
        &self,
        total_questions: usize,
        selected_chapters: Vec<String>,
    ) -> Result<schema::RaesanTest, error::Error> {
        let mcq_count = {
            let mut rng = rand::rng();
            rng.random_range(1..=total_questions)
        };
        let integer_count = total_questions - mcq_count;

        let mut questions: Vec<schema::Question> =
            fetch_questions(&self.db_pool, &selected_chapters, "mcq", mcq_count as i64)
                .await
                .unwrap();
        questions.extend(
            fetch_questions(
                &self.db_pool,
                &selected_chapters,
                "integer",
                integer_count as i64,
            )
            .await
            .unwrap(),
        );

        Ok(schema::RaesanTest {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: chrono::Utc::now().timestamp(),
            total_questions,
            total_mcq_questions: mcq_count,
            total_integer_questions: integer_count,
            questions,
        })
    }
}

async fn fetch_questions(
    db_pool: &sqlite::SqlitePool,
    selected_chapters: &[String],
    question_type: &str,
    question_limit: i64,
) -> Result<Vec<schema::Question>, error::Error> {
    let mut qb = sqlx::QueryBuilder::<sqlx::Sqlite>::new(format!(
        r#"SELECT * FROM question WHERE question_type = '{}' AND chapter_id IN ("#,
        question_type
    ));
    let mut separated = qb.separated(", ");
    for curr_chapter_id in selected_chapters.iter() {
        separated.push_bind(curr_chapter_id);
    }
    separated.push_unseparated(") LIMIT ");
    qb.push_bind(question_limit);
    let query = qb.build_query_as::<schema::Question>();
    Ok(query.fetch_all(db_pool).await.unwrap())
}
