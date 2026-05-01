use crate::state;
use axum::response::IntoResponse;
use std::sync::Arc;

// GET /api/filter_metadata
#[axum::debug_handler]
pub async fn filter_metadata(
    axum::extract::State(server_state): axum::extract::State<Arc<state::ServerState>>,
) -> impl IntoResponse {
    axum::Json(server_state.app.get_filter_metadata().await.unwrap())
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateTestData {
    total_questions: usize,
    selected_chapters: Vec<String>,
}

// POST /api/create_test
#[axum::debug_handler]
pub async fn create_test(
    axum::extract::State(server_state): axum::extract::State<Arc<state::ServerState>>,
    axum::extract::Json(create_test_data): axum::extract::Json<CreateTestData>,
) -> impl IntoResponse {
    axum::Json(
        server_state
            .app
            .create_test(
                create_test_data.total_questions,
                create_test_data.selected_chapters,
            )
            .await
            .unwrap(),
    )
}

// GET /api/chapter_data/{chapter_id}
#[axum::debug_handler]
pub async fn chapter_data(
    axum::extract::State(server_state): axum::extract::State<Arc<state::ServerState>>,
    axum::extract::Path(chapter_id): axum::extract::Path<String>,
) -> impl IntoResponse {
    axum::Json(
        server_state
            .app
            .get_chapter_data(&chapter_id)
            .await
            .unwrap(),
    )
}
