use file::file::{File, FileProperties};

use crate::file_reader::file_reader::FileReader;

pub mod file;
pub mod file_reader;

fn main() {
    let mut file_properties = FileProperties {
        file_name: String::from("./Cargo.lock"),
        is_folder: false,
        path: String::from("./Cargo.lock"),
        file_size: 32.5,
    };

    let mut file = File {
        content: Vec::new(),
        properties: file_properties,
    };
    println!(
        "{}",
        file_reader::file_reader::FileReaderImpl::get_absolute_path(&mut file)
    );
}
