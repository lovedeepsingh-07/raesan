pub const EXAMSIDE_BASE_URL__NAME: &str = "EXAMSIDE_BASE_URL";
pub const FRONTEND_URL__NAME: &str = "FRONTEND_URL";
pub const APP_ENV__NAME: &str = "PUBLIC_APP_ENV";

#[derive(Debug, Clone, PartialEq)]
pub enum Environment {
    DEV,
    PROD,
}
impl From<&str> for Environment {
    fn from(input: &str) -> Self {
        match input {
            "production" => Environment::PROD,
            _ => Environment::DEV,
        }
    }
}
