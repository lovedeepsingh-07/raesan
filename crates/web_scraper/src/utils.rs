#[macro_export]
macro_rules! string_vec {
    ($($input_str:expr),*) => {{
        vec![
            $($input_str.to_string()),*
        ]
    }}
}

pub async fn export_db_to_file(
    db_pool: &sqlx::SqlitePool,
    path: &std::path::Path,
) -> Result<(), sqlx::Error> {
    let path_str = path.to_str().expect("Invalid path");
    sqlx::query("VACUUM INTO ?")
        .bind(path_str)
        .execute(db_pool)
        .await?;
    Ok(())
}
