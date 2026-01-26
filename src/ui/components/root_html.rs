use leptos::prelude::*;

#[component]
pub(crate) fn RootHTML(children: Children) -> impl IntoView {
    view! {
        <html>
            <head>
                <title>raesan</title>
                <link rel="stylesheet" href="/static/tailwind.css"/>
                <script src="/static/scripts/script.js"></script>
            </head>
            <body class="bg-background">{children()}</body>
        </html>
    }
}
