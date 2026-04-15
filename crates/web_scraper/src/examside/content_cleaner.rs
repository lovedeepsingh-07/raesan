pub async fn clean(input: &str) -> Result<String, error::Error> {
    Ok(ammonia::clean(input))
}
