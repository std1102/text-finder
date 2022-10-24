use crate::file::file::{File, FileProperties};
use std::{
    fs::{self},
    io::{BufRead, Read},
    path::PathBuf,
};

use super::result::READ_RESULT;

pub const TRUE: i8 = 1;
pub const FALSE: i8 = 0;
pub const ERROR: i8 = -1;

pub trait FileReader {
    fn get_absolute_path(path: &str) -> READ_RESULT<String>;

    fn get_file_content(file: File) -> Vec<String>;

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
            Err(_) => {
                return READ_RESULT::ERROR;
            }
            Ok(srcdir) => {
                let absolute_path = String::from(srcdir.as_os_str().to_str().unwrap());
                return READ_RESULT::TRUE(absolute_path);
            }
        }
    }

    fn get_file_size(path: &str) -> READ_RESULT<f32> {
        match fs::metadata(path) {
            Ok(md) => {
                let raw_length = md.len();
                READ_RESULT::TRUE((raw_length / 1024) as f32)
            }
            Err(_) => READ_RESULT::ERROR,
        }
    }

    fn get_file_content(file: File) -> Vec<String> {
        let _file = std::fs::File::open(&file.properties.path);
        let mut rs = Vec::new();
        match _file {
            Ok(f) => {
                let reader = std::io::BufReader::new(f);
                reader.lines().for_each(|f| -> () {
                    match f {
                        Ok(line) => {
                            rs.push(line);
                        }
                        Err(e) => {
                            println!("{}", e.kind());
                        }
                    }
                })
            }
            Err(e) => {
                println!("{}", e.kind());
            }
        }
        rs
    }

    fn get_file_name(path: &str) -> READ_RESULT<String> {
        let path_to_file = Self::get_absolute_path(path);
        match path_to_file {
            READ_RESULT::TRUE(_path) => {
                let file = std::path::Path::new(&_path).file_name().unwrap();
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
                let file = File {
                    content: Vec::new(),
                    children: Vec::new(),
                    properties: FileProperties {
                        file_name: Self::get_file_name(path).get_context().to_string(),
                        file_size: Self::get_file_size(path).get_context().to_owned(),
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
        let file = std::fs::File::open(path);
        match file {
            Ok(mut _file) => {
                let mut byte_arr: [u8; 8] = [0; 8];
                _file.read_exact(&mut byte_arr).unwrap_or_else(|e| -> () {
                    println!("{:?}", e);
                });
                match std::str::from_utf8(&byte_arr) {
                    Err(_) => {
                        return READ_RESULT::TRUE(TRUE);
                    }
                    Ok(_) => {
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
