use leptos::prelude::*;

#[component]
pub(crate) fn RootHTML(children: Children) -> impl IntoView {
    view! {
        <html>
            <head>
                <title>raesan</title>
                <link rel="stylesheet" href="/static/tailwind.css"/>
                <script src="https://cdn.jsdelivr.net/npm/htmx.org@2.0.8/dist/htmx.min.js"></script>
            </head>
            <body class="bg-background">{children()}</body>
        </html>
    }
}
