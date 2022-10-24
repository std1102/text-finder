use std::ops::Add;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::Thread;

use crate::file;
use crate::file::file::File;
use crate::file::file::BLOCK_SIZE;
use regex::Regex;

use super::file_reader;
use super::file_reader::FileReader;

pub fn find_text(pattern: String, files: Vec<file::file::File>) {
    let total_files = files.len();
    let total_page = ((total_files / BLOCK_SIZE) as f32).ceil() as usize;
    for i in (0..total_page) {
        let _files = Arc::new(Mutex::new(files[i..i + BLOCK_SIZE].to_vec()));
        let _pattern = Arc::new(Mutex::new(pattern.to_owned()));
        tokio::task::spawn(async move {
            process(&_pattern.lock().unwrap(), &_files.lock().unwrap()).await;
        });
    }
}

async fn process(pattern: &String, mut files: &Vec<File>) {
    let re = Regex::new(&pattern.as_str()).unwrap();
    files.iter().for_each(|mut _file| -> () {
        let mut ref_file = _file.clone();
        ref_file.content = file_reader::FileReaderImpl::get_file_content(_file.clone());
        _file.content.iter().for_each(|content| -> () {
            let mut line_number = 1;
            line_number = line_number + 1;
            match re.is_match(content) {
                true => {
                    let result_line =
                        format!("line :: {} of file {}", line_number, _file.properties.path);
                }
                false => todo!(),
            }
        });
    })
}
