pub mod error;
pub mod pages;
pub mod schema;

pub use pages::chapter_page::extract as decode_chapter_page;
pub use pages::exam_page::extract as decode_exam_page;
pub use pages::metadata::extract as get_page_data;
