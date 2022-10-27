use common::common as SysTime;
extern crate num_cpus;
use file_reader::file_reader::{AsyncFileEmitter, AsyncFileReciever};
use std::{
    env::{self, args},
    sync::{mpsc, Arc},
    thread,
    time::Duration,
    usize,
};

use crate::file_reader::{
    file_reader::{FileReader, FileReaderImpl},
    result::READ_RESULT,
};
pub mod common;
pub mod file;
pub mod file_reader;
mod reactive;
use std::fs;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Missing argument");
        return;
    }

    if args[0].is_empty() && args[1].is_empty() {
        println!("Missing argument");
        return;
    }
    let path = args[1].clone();
    let find_string = args[2].clone();
    let mut sys_thread = num_cpus::get();

    if args.len() >= 4 {
        match args[3].trim().parse::<usize>() {
            Ok(num) => {
                sys_thread = num;
            }
            Err(_) => {
                println!("Invalid format of agument");
                return;
            }
        }
    }

    match file_reader::file_reader::FileReaderImpl::is_exist(&path.clone()) {
        READ_RESULT::TRUE(_) => {}
        READ_RESULT::FALSE => {
            println!("Folder or file not found!");
            return;
        }
        READ_RESULT::ERROR => {
            println!("Unknown error");
            return;
        }
    }

    let (tx, rx) = mpsc::channel();

    let srcdir = PathBuf::from(path);
    let c_path = fs::canonicalize(&srcdir)
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap()
        .to_string();
    println!("Find '{}' in: {}", &find_string, &c_path);
    println!("Number of thread {}", &sys_thread);

    thread::sleep(Duration::from_millis(1500));

    let start_time = SysTime::get_current_milis();
    let t1 = thread::spawn(move || {
        AsyncFileEmitter::emit(tx.clone(), &c_path).await;
    });

    let c_find_string = find_string.clone();
    let t2 = thread::spawn(move || {
        AsyncFileReciever::distribute(rx, 8, c_find_string);
    });

    t1.join().unwrap();
    t2.join().unwrap();
    println!("It takes {}ms", SysTime::get_current_milis() - start_time);
}
