use std::{collections::VecDeque, fs::ReadDir, sync::mpsc::Sender};

use crate::file::file::{File as CustomFile, FileProperties};

use super::file_reader::{self, get_file_metadata, ERROR, FALSE, TRUE};

fn loop_file(transmitter: Sender<CustomFile>, file_dir: ReadDir, queue: &mut VecDeque<CustomFile>) {
    file_dir.for_each(|file_path| {
        let file = get_file_metadata(file_path.unwrap().path().as_os_str().to_str().unwrap());
        if file.properties.is_folder != TRUE {
            transmitter.send(file).unwrap();
        } else {
            queue.push_back(file);
        }
    })
}

// BFS search
pub fn bfs_search(transmitter: Sender<CustomFile>, path: &str) {
    let file = get_file_metadata(path);
    if file.properties.is_folder == ERROR {
        println!("Unknown Error");
        return;
    } else if !file.properties.is_folder == TRUE {
        match transmitter.send(file) {
            Ok(_) => return,
            Err(e) => {
                println!("Sender Error {:?}", e);
                return;
            }
        }
    } else {
        let mut queue: VecDeque<CustomFile> = VecDeque::new();
        queue.push_back(file);
        while queue.len() > 0 {
            let current_node = queue.pop_front().unwrap();
            match std::fs::read_dir(current_node.properties.path) {
                Ok(file_dir) => {
                    loop_file(transmitter.clone(), file_dir, &mut queue);
                }
                Err(_) => {
                    continue;
                }
            }
        }
    }
}

// Recursive
pub fn recursive_search(transmitter: Sender<CustomFile>, path: &str) {
    let file = get_file_metadata(path);
    if file.is_error {
        return;
    } else if file.properties.is_folder != TRUE {
        match transmitter.send(file) {
            Ok(_) => {}
            Err(error_msg) => {
                println!("Error when sending file :: {}", error_msg)
            }
        }
    } else {
        let paths = std::fs::read_dir(file.properties.path);
        match paths {
            Ok(ok_path) => {
                ok_path.into_iter().for_each(|os_path| {
                    recursive_search(
                        transmitter.clone(),
                        os_path.unwrap().path().as_os_str().to_str().unwrap(),
                    );
                });
            }
            Err(_) => {}
        }
    }
}

// TODO DFS search
