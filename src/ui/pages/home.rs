use crate::ui::components::{Button, ButtonVariant, RootHTML};
use leptos::prelude::*;

#[component]
pub(crate) fn Home() -> impl IntoView {
    view! {
        <RootHTML>
            <h1 class="bg-red-500 text-5xl">home page</h1>
            <div>
                <Button class=String::from("text-2xl px-10")>button</Button>
                <Button variant=ButtonVariant::Secondary>button</Button>
                <Button variant=ButtonVariant::Accent>button</Button>
            </div>
        </RootHTML>
    }
}
