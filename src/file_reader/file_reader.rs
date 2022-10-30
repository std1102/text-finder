use std::thread::{self, JoinHandle};

use crate::{
    file::file::{File, FileProperties},
    file_reader::text_finder,
};
use std::{
    path::Path,
    sync::mpsc::{self, Receiver, Sender},
};

use super::{
    result::READ_RESULT,
    search::{bfs_search, recursive_search},
};

pub const TRUE: i8 = 1;
pub const FALSE: i8 = 0;
pub const ERROR: i8 = -1;

pub fn is_exist(path: &str) -> READ_RESULT<i8> {
    match std::path::Path::new(path).exists() {
        true => READ_RESULT::TRUE(TRUE),
        false => READ_RESULT::FALSE,
    }
}

pub fn get_file_metadata(path: &str) -> File {
    let mut file = File {
        is_error: true,
        content: Vec::new(),
        children: Vec::new(),
        properties: FileProperties {
            file_name: String::from("--ERROR--"),
            file_size: 0.0,
            is_folder: ERROR,
            path: String::from("--ERROR--"),
        },
    };
    match is_exist(path) {
        READ_RESULT::TRUE(_) => {
            file.properties.path = path.to_string();
            let _path = Path::new(path);
            if _path.is_dir() {
                file.properties.is_folder = TRUE;
            } else {
                file.properties.is_folder = FALSE;
            }
            match _path.file_name() {
                Some(f_name) => {
                    file.properties.file_name = f_name.to_str().unwrap().to_string();
                }
                None => return file,
            }
            match _path.metadata() {
                Ok(md) => {
                    let file_size = ((md.len() / 1024) as f32).ceil();
                    file.properties.file_size = file_size;
                }
                Err(_) => return file,
            }
            file.is_error = false;
            file
        }
        READ_RESULT::FALSE => file,
        READ_RESULT::ERROR => file,
    }
}

pub struct AsyncFileEmitter {}
pub struct AsyncFileReciever {}

impl AsyncFileEmitter {
    pub fn emit(transmitter: Sender<File>, path: &str) {
        bfs_search(transmitter, path);
    }
}

impl AsyncFileReciever {
    pub fn distribute(reciever: Receiver<File>, thread_size: usize, find_text: String) {
        let mut chanels: Vec<Sender<File>> = Vec::with_capacity(thread_size);
        let mut read_threads: Vec<JoinHandle<()>> = vec![];
        for _ in 0..thread_size {
            let (sender, reciever) = mpsc::channel();
            chanels.push(sender);
            let c_string = find_text.clone();
            let t = thread::spawn(move || text_finder::find_text(reciever, &c_string));
            read_threads.push(t);
        }
        let mut message_index = 0;
        loop {
            match reciever.recv() {
                Ok(file) => {
                    message_index = message_index + 1;
                    match chanels[&message_index % thread_size].send(file) {
                        Ok(_) => continue,
                        Err(e) => {
                            continue;
                        }
                    }
                }
                Err(_) => break,
            }
        }
        for thread in read_threads {
            thread.join().unwrap();
        }
        println!("Total file scanned {}", &message_index);
        return;
    }
}
