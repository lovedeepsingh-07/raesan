mod api;
mod static_files;

// re-exports
pub use api::route as api;
pub use static_files::route as static_files;

use axum::response::IntoResponse;
use frontend::pages;
use leptos::prelude::RenderHtml;

pub async fn home() -> impl IntoResponse {
    axum::response::Html(pages::Home().to_html()).into_response()
}
