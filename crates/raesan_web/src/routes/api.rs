use crate::state;
use axum::response::IntoResponse;
use std::sync::Arc;

// GET /api/filter_metadata
pub async fn filter_metadata(
    axum::extract::State(server_state): axum::extract::State<Arc<state::ServerState>>,
) -> impl IntoResponse {
    axum::Json(server_state.app.get_filter_metadata().await.unwrap())
}

// POST /api/create_test
pub async fn create_test(
    axum::extract::State(server_state): axum::extract::State<Arc<state::ServerState>>,
    axum::extract::Json(create_test_data): axum::extract::Json<serde_json::Value>,
) -> impl IntoResponse {
    let _ = server_state;
    log::info!("{:#?}", create_test_data);
    "CREATING_TEST".into_response()
}

// GET /api/chapter_data/{chapter_id}
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
