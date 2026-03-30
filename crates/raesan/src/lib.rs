use sqlx::sqlite;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum Environment {
    DEV,
    PROD,
}
impl From<&str> for Environment {
    fn from(input: &str) -> Self {
        match input {
            "production" => Environment::PROD,
            _ => Environment::DEV,
        }
    }
}

#[derive(Debug)]
pub struct App {
    pub env: Environment,
    pub db_pool: sqlite::SqlitePool,
}

impl App {
    pub async fn new(db_path: &str, env: Environment) -> Result<Self, error::Error> {
        let db_options = sqlite::SqliteConnectOptions::from_str(db_path)?.create_if_missing(false);
        let db_pool = sqlite::SqlitePoolOptions::new()
            .connect_with(db_options)
            .await?;

        Ok(Self { env, db_pool })
    }

    // pub async fn get_exam_list(&self) -> Result<Vec<scheam::Exam>, error::Error> {
    //     let output = sqlx::query_as::<_, schema::Exam>(schema::_Exam::LIST_QUERY)
    //         .fetch_all(&self.db_pool)
    //         .await?;
    //     Ok(output)
    // }
    //
    // pub async fn get_subject_list(&self) -> Result<Vec<tree_schema::T_Subject>, error::Error> {
    //     let mut output: Vec<tree_schema::T_Subject> = Vec::new();
    //
    //     let exam_list = self.get_exam_list().await?;
    //     for curr_exam in exam_list {
    //         let curr_exam_subjects: Vec<tree_schema::T_Subject> =
    //             sqlx::query_as::<_, tree_schema::T_Subject>(tree_schema::T_Subject::LIST_QUERY)
    //                 .bind(&curr_exam.id)
    //                 .fetch_all(&self.db_pool)
    //                 .await?;
    //         output.extend(curr_exam_subjects);
    //     }
    //     Ok(output)
    // }
    //
    // pub async fn get_chapter_list(&self) -> Result<Vec<tree_schema::T_Chapter>, error::Error> {
    //     let output: Vec<tree_schema::T_Chapter> = Vec::new();
    //
    //     // let subject_list = self.get_subject_list().await?;
    //     // for curr_subject in subject_list {
    //     //     let curr_subject_chapters: Vec<tree_schema::T_Chapter> =
    //     //         sqlx::query_as::<_, tree_schema::T_Chapter>(Self::CHAPTER_LIST_QUERY)
    //     //             .bind(&curr_subject.id)
    //     //             .fetch_all(&self.db_pool)
    //     //             .await?;
    //     //     output.extend(curr_subject_chapters);
    //     // }
    //     Ok(output)
    // }
}
