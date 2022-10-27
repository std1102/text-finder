use regex::Regex;

use crate::file::file::File as CustomFile;
use std::fs::File as OsFile;
use std::io::{self, BufRead};
use std::sync::mpsc::Receiver;

pub fn find_text(recieve: Receiver<CustomFile>, find_str: &String) {
    let re = Regex::new(find_str).unwrap();
    loop {
        match recieve.recv() {
            Ok(file) => {
                let os_file = OsFile::open(&file.properties.path);
                match os_file {
                    Ok(o_file) => {
                        let mut line_number = 1;
                        let reader = io::BufReader::new(o_file);
                        reader.lines().into_iter().for_each(|l| match l {
                            Ok(line_string) => {
                                if re.is_match(&line_string) {
                                    println!(
                                        "String found at file :: {} | line :: {}",
                                        &file.properties.path, &line_number
                                    );
                                    line_number = line_number + 1;
                                }
                            }
                            Err(_) => {
                                return;
                            }
                        });
                    }
                    Err(_) => {}
                }
            }
            Err(err) => {
                break;
            }
        }
    }
}
