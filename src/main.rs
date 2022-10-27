use common::common as SysTime;
use file_reader::file_reader::{AsyncFileEmitter, AsyncFileReciever};
use std::{
    env::{self, args},
    sync::{mpsc, Arc},
    thread,
};

use crate::file_reader::{file_reader::FileReader, result::READ_RESULT};
pub mod common;
pub mod file;
pub mod file_reader;
mod reactive;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = args[1].clone();
    let find_string = args[2].clone();

    match file_reader::file_reader::FileReaderImpl::is_exist(&path.clone()) {
        READ_RESULT::TRUE(_) => {}
        READ_RESULT::FALSE => {
            println!("Folder or file not found!");
        }
        READ_RESULT::ERROR => {
            println!("Unknown error");
        }
    }

    let start_time = SysTime::get_current_milis();
    let (tx, rx) = mpsc::channel();
    let c_path = path.clone();
    let t1 = thread::spawn(move || {
        AsyncFileEmitter::emit(tx.clone(), &c_path);
    });

    let c_find_string = find_string.clone();
    let t2 = thread::spawn(move || {
        AsyncFileReciever::distribute(rx, 8, c_find_string);
    });

    t1.join().unwrap();
    t2.join().unwrap();
    println!("It takes {}ms", SysTime::get_current_milis() - start_time);
}
