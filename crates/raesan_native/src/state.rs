#[derive(Debug)]
pub struct AppState {
    pub app: raesan::App,
}

impl AppState {
    pub async fn new(db_path: &str) -> Result<Self, error::Error> {
        let app_env = match std::env::var("PUBLIC_APP_ENV") {
            Ok(out) => out,
            Err(_) => {
                log::warn!("Unable to get the app environment, assuming development environment");
                String::from("development")
            }
        };
        let app = raesan::App::new(db_path, raesan::Environment::from(app_env.as_str())).await?;

        Ok(Self { app })
    }
}
