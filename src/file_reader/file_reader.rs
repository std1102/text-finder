use crate::file::file::{File, FileProperties};
use std::{
    f32::consts::E,
    fs::{self, Metadata},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use super::exception::READ_RESULT;

pub const TRUE: i8 = 1;
pub const FALSE: i8 = 0;
pub const ERROR: i8 = -1;
pub const BLOCK_SIZE: usize = 8;

pub trait FileReader {
    fn get_absolute_path(path: &str) -> READ_RESULT<String>;

    fn get_file_content(path: &str) -> Vec<String>;

    fn get_file_name(path: &str) -> READ_RESULT<String>;

    fn get_file_size(path: &str) -> READ_RESULT<f32>;

    fn get_meta_data_recursively(path: &str) -> Vec<File>;

    fn is_binary(path: &str) -> READ_RESULT<i8>;

    fn is_exist(path: &str) -> READ_RESULT<i8>;

    fn is_folder(path: &str) -> READ_RESULT<i8>;
}

pub struct FileReaderImpl;

impl FileReader for FileReaderImpl {
    fn is_folder(path: &str) -> READ_RESULT<i8> {
        let md = fs::metadata(path).unwrap();
        match md.is_dir() {
            true => READ_RESULT::TRUE(TRUE),
            false => READ_RESULT::FALSE,
        }
    }

    fn is_exist(path: &str) -> READ_RESULT<i8> {
        if std::path::Path::new(path).exists() {
            return READ_RESULT::TRUE(TRUE);
        }
        READ_RESULT::FALSE
    }

    fn get_absolute_path(path: &str) -> READ_RESULT<String> {
        let src_dir = PathBuf::from(path).canonicalize();
        match src_dir {
            Err(e) => {
                return READ_RESULT::ERROR;
            }
            Ok(srcdir) => {
                let absolute_path = String::from(srcdir.as_os_str().to_str().unwrap());
                return READ_RESULT::TRUE(absolute_path);
            }
        }
    }

    fn get_file_size(path: &str) -> READ_RESULT<f32> {
        todo!()
    }

    fn get_file_content(path: &str) -> Vec<String> {
        todo!()
    }

    fn get_file_name(path: &str) -> READ_RESULT<String> {
        let path_to_file = Self::get_absolute_path(path);
        match path_to_file {
            READ_RESULT::TRUE(path) => {
                let file = std::path::Path::new(&path).file_name().unwrap();
                READ_RESULT::TRUE(String::from(file.to_str().unwrap()))
            }
            READ_RESULT::FALSE => todo!(),
            READ_RESULT::ERROR => todo!(),
        }
    }

    fn get_meta_data_recursively(path: &str) -> Vec<File> {
        let mut result: Vec<File> = Vec::new();
        match Self::is_exist(path) {
            READ_RESULT::FALSE => {
                return result;
            }
            READ_RESULT::TRUE(_) => {
                let mut file = File {
                    content: Vec::new(),
                    children: Vec::new(),
                    properties: FileProperties {
                        file_name: Self::get_file_name(path).get_context().to_string(),
                        file_size: 0.0,
                        is_binary: match Self::is_binary(path) {
                            READ_RESULT::TRUE(_) => TRUE,
                            READ_RESULT::FALSE => FALSE,
                            READ_RESULT::ERROR => ERROR,
                        },
                        is_folder: match Self::is_folder(path) {
                            READ_RESULT::TRUE(_) => TRUE,
                            READ_RESULT::FALSE => FALSE,
                            READ_RESULT::ERROR => ERROR,
                        },
                        path: Self::get_absolute_path(path).get_context().to_string(),
                    },
                };

                if file.properties.is_folder != TRUE {
                    if file.properties.is_binary == TRUE {
                        return result;
                    } else {
                        println!("{}", &file.properties.path);
                        result.push(file);
                        return result;
                    }
                } else {
                    let paths = fs::read_dir(file.properties.path).unwrap();
                    paths.into_iter().for_each(|p| -> () {
                        result.append(&mut Self::get_meta_data_recursively(
                            p.unwrap().path().as_os_str().to_str().unwrap(),
                        ))
                    });
                    return result;
                }
            }
            READ_RESULT::ERROR => return result,
        };
    }

    // TODO FIX THIS ERROR
    fn is_binary(path: &str) -> READ_RESULT<i8> {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .read(true)
            .open(path);
        match file {
            Ok(_file) => {
                let reader = BufReader::new(_file);
                match reader.lines().next() {
                    None => {
                        return READ_RESULT::TRUE(TRUE);
                    }
                    Some(_) => {
                        return READ_RESULT::FALSE;
                    }
                };
            }
            Err(_) => {
                return READ_RESULT::ERROR;
            }
        }
    }
}
