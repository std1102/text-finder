use file::file::File;
use file_reader::file_reader::FileReaderImpl;

use crate::file_reader::{file_reader::FileReader, text_finder};

pub mod common;
pub mod file;
pub mod file_reader;

fn main() {
    let start_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("ASD")
        .as_millis();
    let files: Vec<File> =
        FileReaderImpl::get_meta_data_recursively(r#"D:\Project\text-finder\test"#);
    println!("total files {}", &files.len());
    text_finder::find_text(String::from("6789553615590802434"), files);
    println!(
        "IT TAKES {}",
        (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("ASD")
            .as_millis()
            - start_time)
    );
}
