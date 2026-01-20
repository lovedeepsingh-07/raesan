use axum::response::IntoResponse;
use tokio::sync::mpsc;
use leptos::{html::ElementChild, prelude::RenderHtml, attr::{global::ClassAttribute, custom::CustomAttribute}};

pub const SERVER_ADDRESS: &'static str = "0.0.0.0:3000";
pub const STATIC_FOLDER: &'static str = "static";
pub const COMMAND_CAP: usize = 64;

#[leptos::component]
fn Button(children: leptos::children::Children) -> impl leptos::IntoView {
    leptos::view! {
      <button
        hx-get="/api"
        class="bg-red-500 text-black rounded-lg border-2 border-black px-4 py-2 hover:opacity-75"
        hx-swap="none"
        >{children()}</button>
    }
}

#[leptos::component]
fn App() -> impl leptos::IntoView {
    leptos::view! {
        <html>
            <head>
                <title>raesan</title>
                <link rel="stylesheet" href="/static/tailwind.css"/>
                <script src="https://cdn.jsdelivr.net/npm/htmx.org@2.0.8/dist/htmx.min.js"></script>
            </head>
            <body>
                <h1 class="bg-red-500 text-5xl">hello, world!</h1>
                <Button>"button"</Button>
                <Button>"another button"</Button>
            </body>
        </html>
    }
}

async fn root() -> impl IntoResponse {
    axum::response::Html(App().to_html()).into_response()
}

async fn static_files(
    axum::extract::Path(file_path): axum::extract::Path<String>,
) -> impl IntoResponse {
    let static_folder = std::path::PathBuf::from(STATIC_FOLDER);
    if static_folder.try_exists().unwrap_or_else(|_| false) {
        let guess = mime_guess::from_path(&file_path);

        let file_path = static_folder.join(file_path);
        if file_path.try_exists().unwrap_or_else(|_| false) {
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

async fn api(axum::extract::State(tx): axum::extract::State<mpsc::Sender<Command>>) -> impl IntoResponse {
    tx.send(Command::API).await.unwrap();
    String::from("API_RESPONSE")
}

#[derive(Debug)]
pub enum Command {
    API
}

#[tokio::main]
async fn main() {
    let logger_env = env_logger::Env::default().filter_or("RUST_LOG", "raesan=debug");
    env_logger::init_from_env(logger_env);

    let (tx, mut rx) = mpsc::channel::<Command>(COMMAND_CAP);
    tokio::spawn(async move {
        while let Some(command) = rx.recv().await {
            log::debug!("API request with command: {:#?}", command);
        }
    });

    let router = axum::Router::new()
        .route("/", axum::routing::get(root))
        .route("/api", axum::routing::get(api))
        .route("/static/{*file_path}", axum::routing::get(static_files)).with_state(tx);

    let listener = tokio::net::TcpListener::bind(SERVER_ADDRESS).await.unwrap();
    log::info!("server started on {}", SERVER_ADDRESS);
    axum::serve(listener, router).await.unwrap();
}
