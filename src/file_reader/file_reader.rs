use crate::file;
use std::{
    f32::consts::E,
    fs::{self, Metadata},
    path::PathBuf,
};

const BLOCK_SIZE: usize = 8;

pub trait FileReader {
    fn get_file_size(file: &mut file::file::File) -> f32;

    fn get_file_content(file: &mut file::file::File) -> Vec<String>;

    fn get_file_name(file: &mut file::file::File) -> String;

    fn is_folder(file: &mut file::file::File) -> bool {
        if !Self::is_exist(file) {
            return false;
        }
        let md = fs::metadata(&file.properties.path).unwrap();
        match md.is_dir() {
            true => {
                file.properties.is_folder = true;
                return true;
            }
            false => {
                file.properties.is_folder = false;
                return false;
            }
        }
    }

    fn is_exist(file: &mut file::file::File) -> bool {
        let read_result = fs::read(&file.properties.path[..]);
        match read_result {
            Err(_) => {
                if Self::get_absolute_path(file).is_empty() {
                    println!("FILE_NOT_FOUND!");
                    return false;
                }
                true
            }
            Ok(_) => true,
        }
    }

    fn get_absolute_path(file: &mut file::file::File) -> String {
        let src_dir = PathBuf::from(&file.properties.path[..])
            .canonicalize()
            .unwrap_or_else(|e| -> PathBuf {
                println!("{:?}", e);
                return PathBuf::new();
            });
        let absolute_path = String::from(src_dir.as_os_str().to_str().unwrap());
        file.properties.path = absolute_path.clone();
        absolute_path
    }
}

pub struct FileReaderImpl;

impl FileReader for FileReaderImpl {
    fn get_file_size(file: &mut file::file::File) -> f32 {
        todo!()
    }

    fn get_file_content(file: &mut file::file::File) -> Vec<String> {
        todo!()
    }

    fn get_file_name(file: &mut file::file::File) -> String {
        todo!()
    }
}
