use regex::Regex;

use crate::file::file::File as CustomFile;
use std::collections::VecDeque;
use std::fs::File as OsFile;
use std::io::{self, BufRead};
use std::sync::mpsc::Receiver;

// search file include binary file
// add waiting queue here
pub fn find_text(recieve: Receiver<CustomFile>, find_str: &String) {
    let re = Regex::new(find_str).unwrap();
    let mut waiting_queue: VecDeque<CustomFile> = VecDeque::new();
    loop {
        match recieve.try_recv() {
            Ok(file) => {
                waiting_queue.push_back(file);
                let current_file = waiting_queue.pop_front().unwrap();
                let os_file = OsFile::open(&current_file.properties.path);
                match os_file {
                    Ok(o_file) => {
                        let mut line_number = 1;
                        let reader = io::BufReader::new(o_file);
                        reader.lines().for_each(|line| match line {
                            Ok(line_string) => {
                                if re.is_match(&line_string) {
                                    println!(
                                        "{} | line :: {}",
                                        &current_file.properties.path.trim_start_matches(r"\\?\"),
                                        &line_number
                                    );
                                    line_number = line_number + 1;
                                }
                            }
                            Err(_) => return,
                        });
                    }
                    Err(_) => continue,
                }
            }
            Err(_) => {
                return;
            }
        }
    }
}
