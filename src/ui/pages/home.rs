use crate::ui::components::{Button, RootHTML};
use leptos::prelude::*;

#[component]
pub(crate) fn Home() -> impl IntoView {
    view! {
        <RootHTML>
            <Button class=String::from("text-2xl px-10") on_click=String::from("window.send_api_request()")>button</Button>
        </RootHTML>
    }
}
