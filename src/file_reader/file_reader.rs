use queues::*;
use std::thread;
extern crate queues;

use crate::{
    common,
    file::file::{File, FileProperties},
    file_reader::text_finder,
};
use std::{
    fs::{self, metadata},
    io::{BufRead, ErrorKind, Read},
    path::{Path, PathBuf},
    sync::mpsc::{self, Receiver, Sender},
    thread::Thread,
};

use common::common as SysTime;

use super::result::READ_RESULT;

pub const TRUE: i8 = 1;
pub const FALSE: i8 = 0;
pub const ERROR: i8 = -1;

pub trait FileReader {
    fn get_meta_data_recursively(path: &str) -> Vec<File>;

    fn is_exist(path: &str) -> READ_RESULT<i8> {
        match std::path::Path::new(path).exists() {
            true => READ_RESULT::TRUE(TRUE),
            false => READ_RESULT::FALSE,
        }
    }

    fn get_file_info(path: &str) -> File {
        let mut file = File {
            is_error: true,
            content: Vec::new(),
            children: Vec::new(),
            properties: FileProperties {
                file_name: String::from("--ERROR--"),
                file_size: 0.0,
                is_binary: ERROR,
                is_folder: ERROR,
                path: String::from("--ERROR--"),
            },
        };
        match Self::is_exist(path) {
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
                    Err(e) => return file,
                }
                file.is_error = false;
                file
            }
            READ_RESULT::FALSE => file,
            READ_RESULT::ERROR => file,
        }
    }
}

pub struct FileReaderImpl {}

impl FileReader for FileReaderImpl {
    fn get_meta_data_recursively(path: &str) -> Vec<File> {
        let mut result: Vec<File> = Vec::new();
        let file = Self::get_file_info(path);

        if file.is_error {
            return result;
        } else if file.properties.is_folder != TRUE {
            result.push(file);
        } else {
            let paths = fs::read_dir(file.properties.path).unwrap();
            paths.for_each(|p| -> () {
                result.append(&mut Self::get_meta_data_recursively(
                    p.unwrap().path().as_os_str().to_str().unwrap(),
                ))
            });
        }
        return result;
    }
}

pub struct AsyncFileEmitter {}
pub struct AsyncFileReciever {}

impl AsyncFileEmitter {
    pub fn emit(transmitter: Sender<File>, path: &str) {
        let start_time = SysTime::get_current_milis();
        Self::interval_file(transmitter, path);
        println!(
            "ASYNC TASK TAKES {}",
            SysTime::get_current_milis() - start_time
        );
    }

    pub fn interval_file(transmitter: Sender<File>, path: &str) {
        let mut file = FileReaderImpl::get_file_info(path);
        if file.is_error {
            return;
        } else if file.properties.is_folder != TRUE {
            match transmitter.send(file) {
                Ok(file_msg) => {
                    return;
                }
                Err(error_msg) => {
                    println!("ERROR FROM TRANSMITTER {:?}", error_msg);
                }
            }
        } else {
            let paths = fs::read_dir(file.properties.path);
            match paths {
                Ok(ok_path) => {
                    ok_path.into_iter().for_each(|os_path| {
                        Self::interval_file(
                            transmitter.clone(),
                            os_path.unwrap().path().as_os_str().to_str().unwrap(),
                        );
                    });
                }
                Err(err) => {
                    return;
                }
            }
        }
    }
}

impl AsyncFileReciever {
    pub fn distribute(recieve: Receiver<File>, thread_size: usize) {
        let mut queues: Vec<Queue<File>> = Vec::with_capacity(thread_size);
        let mut chanels: Vec<Sender<File>> = Vec::with_capacity(thread_size);
        for i in 0..thread_size {
            queues.push(Queue::new());
            let (sender, reciever) = mpsc::channel();
            chanels.push(sender);
            thread::spawn(move || text_finder::find_text(reciever));
        }
        let mut index = 0;
        loop {
            match recieve.recv() {
                Ok(file) => {
                    index = index + 1;
                    match chanels[&index % thread_size].send(file) {
                        Ok(ok) => continue,
                        Err(e) => continue,
                    }
                    // match queues.iter_mut().min_by(|a, b| a.size().cmp(&b.size())) {
                    //     Some(quiu) => {
                    //         // quiu.add(file).unwrap();
                    //         println!("{}", &file.properties.path);
                    //         match chanels[&index % thread_size].0.send(file) {
                    //             Ok(ok) => continue,
                    //             Err(e) => continue,
                    //         }
                    //     }
                    //     None => {
                    //         println!("NOT FOUND QUEUE");
                    //         continue;
                    //     }
                    // }
                }
                Err(err) => {
                    println!("ERROR FROM RECIEVER {:?}", err);
                    break;
                }
            }
        }
        println!("TOTAL MESSAGE {}", index);
    }
}
