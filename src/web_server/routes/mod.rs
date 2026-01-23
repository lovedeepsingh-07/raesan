mod api;
mod static_files;

pub(crate) use api::route as api;
pub(crate) use static_files::route as static_files;

use crate::ui::pages;
use axum::response::IntoResponse;
use leptos::prelude::RenderHtml;

pub(crate) async fn home() -> impl IntoResponse {
    axum::response::Html(pages::Home().to_html()).into_response()
}
