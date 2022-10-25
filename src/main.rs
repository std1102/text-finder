use std::io::BufRead;

use file::file::File;
use file_reader::file_reader::FileReaderImpl;
use rayon::{prelude::IntoParallelRefIterator, ThreadPoolBuilder};

use crate::file_reader::{file_reader::FileReader, text_finder};

pub mod common;
pub mod file;
pub mod file_reader;

fn main() {
    let start_time = common::common::get_current_milis();
    let files: Vec<File> =
        FileReaderImpl::get_meta_data_recursively(&r#"C:\Users\luyen\Desktop\Projects"#);
    println!("total files {}", &files.len());
    let t_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(100)
        .build()
        .unwrap();
    for mut i in (0..files.len() - 3) {
        t_pool.install(|| -> () {
            files[i..i + 2].iter().for_each(|f| -> () {
                let f = std::fs::File::open(&f.properties.path);
                match f {
                    Ok(_f) => {
                        let reader = std::io::BufReader::new(_f);
                        reader.lines().for_each(|line| -> () {
                            match line {
                                Ok(string) => {let a = string;},
                                Err(e) => {let a = e;},
                            }
                        })
                    }
                    Err(_) => todo!(),
                }
            });
        })
    }
}
