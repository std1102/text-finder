use file::file::File;
use file_reader::file_reader::FileReaderImpl;
use rayon::ThreadPoolBuilder;

use crate::file_reader::{file_reader::FileReader, text_finder};

pub mod common;
pub mod file;
pub mod file_reader;

fn main() {
    let start_time = common::common::get_current_milis();
    let fr = FileReaderImpl {
        rayon_thread_pool: ThreadPoolBuilder::new().num_threads(100).build().unwrap(),
    };
    let files: Vec<File> = FileReaderImpl::get_meta_data_recursively(&r#"../../"#);
    println!("total files {}", &files.len());
    println!(
        "takes {}ms",
        (common::common::get_current_milis() - start_time)
    );
}
