use file::file::{File, FileProperties};
use file_reader::file_reader::FileReaderImpl;

use crate::file_reader::file_reader::FileReader;

pub mod common;
pub mod file;
pub mod file_reader;

fn main() {
    let start_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("ASD")
        .as_millis();
    let files: Vec<File> = FileReaderImpl::get_meta_data_recursively(r#"C:\Users\luyen\Desktop\Projects\Rs\text-finder\src"#);
    println!("total files {}", &files.len());
    println!(
        "IT TAKES {}",
        (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("ASD")
            .as_millis()
            - start_time)
    );
}
