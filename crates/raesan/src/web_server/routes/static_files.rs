use crate::constants;
use axum::response::IntoResponse;

pub async fn route(
    axum::extract::Path(file_path): axum::extract::Path<String>,
) -> impl IntoResponse {
    let static_folder = std::path::PathBuf::from(constants::STATIC_FOLDER);
    if static_folder.try_exists().unwrap_or(false) {
        let guess = mime_guess::from_path(&file_path);

        let file_path = static_folder.join(file_path);
        if file_path.try_exists().unwrap_or(false) {
            let file_content = std::fs::read_to_string(file_path).unwrap();

            return (
                [("Content-Type", guess.first().unwrap().to_string())],
                file_content,
            )
                .into_response();
        }
        return String::from("DOES NOT EXIST").into_response();
    }
    return String::from("DOES NOT EXIST").into_response();
}
