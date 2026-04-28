#[derive(Debug)]
pub struct AppState {
    pub app: raesan::App,
}

impl AppState {
    pub async fn new(db_path: &str, app_env: raesan::Environment) -> Result<Self, error::Error> {
        Ok(Self {
            app: raesan::App::new(db_path, app_env).await?,
        })
    }
}
