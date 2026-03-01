#[macro_export]
macro_rules! string_vec {
    ($($input_str:expr),*) => {{
        vec![
            $($input_str.to_string()),*
        ]
    }}
}
