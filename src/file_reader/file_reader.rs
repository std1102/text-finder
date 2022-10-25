use rayon::{ThreadPool, ThreadPoolBuilder};

use crate::file::file::{File, FileProperties};
use std::{
    fs::{self, metadata},
    io::{BufRead, ErrorKind, Read},
    path::{Path, PathBuf},
    thread::Thread,
};

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

pub struct FileReaderImpl {
    pub rayon_thread_pool: ThreadPool,
}

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
