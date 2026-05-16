#[macro_export]
macro_rules! string_vec {
    ($($input_str:expr),*) => {{
        vec![
            $($input_str.to_string()),*
        ]
    }}
}

pub fn contains_image(input: &str) -> bool {
    input.contains("https")
        || input.contains("jpg")
        || input.contains("jpeg")
        || input.contains("png")
        || input.contains("cdn")
}
